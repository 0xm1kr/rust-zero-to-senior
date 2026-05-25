use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "borrow-checker-fights",
            category: "Senior Pitfalls",
            title: "Common Borrow-Checker Battles",
            description: r#"<p>These are the four patterns that trip up experienced engineers coming from GC'd languages. Knowing the idiomatic fix — not just the rule — is what separates senior Rust engineers.</p>
<h3>Pattern 1: Iterating a Vec while mutating it</h3>
<p>You cannot hold an iterator over a <code>Vec</code> and push/remove from it in the same scope. Fix: collect the indices you want to act on, then mutate in a second pass.</p>
<h3>Pattern 2: Borrow outliving a mutation</h3>
<p><code>let x = &amp;v[0]; v.push(...)</code> — the push might reallocate, invalidating <code>x</code>. Fix: if <code>T: Copy</code>, copy the value. Otherwise, drop the borrow first, or clone.</p>
<h3>Pattern 3: Returning a reference to a local</h3>
<p>You cannot return <code>&amp;local_var</code> — it is dropped at the end of the function. Fix: return an owned value, or take a reference parameter and return a borrow of <em>that</em>.</p>
<h3>Pattern 4: Two mutable borrows into one slice</h3>
<p><code>let a = &amp;mut v[0]; let b = &amp;mut v[1];</code> — the compiler cannot prove they are disjoint. Fix: <code>slice::split_at_mut</code> which uses unsafe internally to give you two non-overlapping mutable slices.</p>"#,
            code: r#"fn main() {
    // --- Pattern 1: Iterate + mutate — collect indices first, then act ---
    // WRONG (compile error): cannot borrow `v` as mutable while iterating it
    //   for x in &v { if *x > 2 { v.push(0); } }
    //
    // FIX: collect the indices you want, then mutate in a separate pass.
    let mut v = vec![1, 2, 3, 4, 5];
    let to_scale: Vec<usize> = (0..v.len())
        .filter(|&i| v[i] % 2 == 0)
        .collect();
    for i in to_scale {
        v[i] *= 10;
    }
    println!("After scaling evens x10: {:?}", v);

    // --- Pattern 2: Copy the value before mutating ---
    // WRONG: let first = &data[0]; data.push(99); println!("{}", first);
    //   (first borrows data; push may reallocate and invalidate the borrow)
    //
    // FIX: i32 is Copy — copy the value, no live borrow remains.
    let mut data = vec![10i32, 20, 30];
    let first = data[0]; // Copy — value copied, not borrowed
    data.push(99);       // fine — no outstanding reference
    println!("first={}, data={:?}", first, data);

    // --- Pattern 3: Return a borrow of the PARAMETER, not a local ---
    // WRONG: fn bad() -> &str { let s = "x".to_string(); &s }
    //   (s is dropped when the function returns)
    //
    // FIX A — return owned:
    fn first_word_owned(s: &str) -> String {
        s.split_whitespace().next().unwrap_or("").to_owned()
    }
    // FIX B — borrow the parameter (lifetime elision handles the annotation):
    fn first_word_borrowed(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    println!("owned  : {}", first_word_owned("hello world"));
    println!("borrow : {}", first_word_borrowed("hello world"));

    // --- Pattern 4: Two mutable slices — use split_at_mut ---
    // WRONG: let a = &mut arr[0]; let b = &mut arr[1];
    //   (cannot borrow arr as mutable more than once)
    //
    // FIX: split_at_mut gives two non-overlapping &mut slices.
    let mut arr = [1i32, 2, 3, 4, 5];
    let (left, right) = arr.split_at_mut(2);
    left[0]  = 100;
    right[0] = 300; // arr[2]
    println!("split_at_mut result: {:?}", arr);
}"#,
            notes: vec![
                "Collect-then-mutate is the canonical fix for 'iterate + mutate'. The second pass has no live iterator.",
                "Copy types (i32, bool, char, …) avoid the borrow entirely — the value is copied, not referenced.",
                "split_at_mut is the stdlib answer to 'I need two mutable slices'. It uses unsafe internally but is safe to call.",
                "Returning a &str from a fn is fine when it borrows from a parameter — lifetime elision handles this case.",
                "Interview tip: demonstrate you know WHY the borrow checker rejects these, not just what the error message says.",
            ],
        },
        Lesson {
            id: "interior-mutability",
            category: "Senior Pitfalls",
            title: "Interior Mutability: Cell, RefCell, Mutex",
            description: r#"<p>Rust's borrow checker enforces exclusive mutation at compile time. <b>Interior mutability</b> is the escape hatch: it moves the borrow check to runtime (or uses tricks like <code>Cell</code> that never need runtime checks at all).</p>
<h3>Cell&lt;T&gt; — Copy types, no borrows</h3>
<p><code>std::cell::Cell</code> allows mutation through a shared reference for <code>Copy</code> types. No references in or out. Never panics. Zero overhead — uses a <code>UnsafeCell</code> under the hood but is safe to use.</p>
<h3>RefCell&lt;T&gt; — any type, runtime borrow checking</h3>
<p>Tracks borrow counts at runtime. <code>.borrow()</code> → <code>Ref&lt;T&gt;</code> (shared). <code>.borrow_mut()</code> → <code>RefMut&lt;T&gt;</code> (exclusive). Panics at runtime if the borrow rules are violated.</p>
<h3>Mutex&lt;T&gt; — thread-safe interior mutability</h3>
<p>Like <code>RefCell</code> but works across threads. <code>.lock().unwrap()</code> returns a <code>MutexGuard&lt;T&gt;</code>. Blocks if another thread holds the lock. Poison on panic.</p>
<h3>Common combo: Rc&lt;RefCell&lt;T&gt;&gt;</h3>
<p>Graph-like single-threaded structures where multiple owners need mutation. Trades compile-time guarantees for runtime checks. Prefer when fighting the borrow checker costs more than the runtime overhead.</p>"#,
            code: r#"use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct Counter {
    // RefCell: any type, runtime borrow checking, single thread
    value: RefCell<i32>,
    // Cell: Copy types only, no borrows in or out, never panics
    fast:  Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
            fast:  Cell::new(0),
        }
    }

    // &self (not &mut self) — mutation goes through interior mutability
    fn increment(&self) {
        *self.value.borrow_mut() += 1; // runtime borrow check
        self.fast.set(self.fast.get() + 1); // no borrow — just get/set
    }

    fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

fn main() {
    let c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    println!("RefCell value : {}", c.get());
    println!("Cell fast     : {}", c.fast.get());

    // --- Rc<RefCell<T>>: shared ownership + mutation, single thread ---
    // Classic pattern for graph nodes, trees with back-edges, etc.
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let alias  = Rc::clone(&shared);        // second owner

    alias.borrow_mut().push(4);             // mutate through one handle
    shared.borrow_mut().push(5);            // mutate through the other
    println!("Shared vec    : {:?}", shared.borrow());

    // --- What panics look like (commented out — would panic at runtime) ---
    // let _a = shared.borrow();
    // let _b = shared.borrow_mut(); // PANIC: already borrowed

    // --- For thread-safe work, use Arc<Mutex<T>> instead ---
    use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(0i32));
    {
        let mut guard = counter.lock().unwrap();
        *guard += 10;
    } // guard dropped here → lock released
    println!("Mutex counter : {}", counter.lock().unwrap());
}"#,
            notes: vec![
                "Cell<T>: T must be Copy. No references in or out. get()/set(). Zero overhead, never panics.",
                "RefCell<T>: any T. borrow() / borrow_mut() check at runtime — panics on violation.",
                "Rc<RefCell<T>>: multiple owners + mutation on one thread. The go-to for graph-like structures.",
                "Arc<Mutex<T>>: same idea but thread-safe. Replace Rc with Arc and RefCell with Mutex.",
                "Interview tip: explain when each is appropriate — Cell for hot counters, RefCell for complex ownership, Mutex for threads.",
            ],
        },
        Lesson {
            id: "unsafe-and-fnsync",
            category: "Senior Pitfalls",
            title: "Unsafe, Send/Sync Gotchas, FFI",
            description: r#"<p><code>unsafe</code> grants exactly <b>five extra capabilities</b>. The borrow checker and type system still apply inside an <code>unsafe</code> block — you are NOT disabling Rust's safety guarantees, only opting in to operations the compiler cannot verify automatically.</p>
<h3>The five unsafe superpowers</h3>
<ol>
  <li>Dereference raw pointers (<code>*const T</code>, <code>*mut T</code>)</li>
  <li>Call <code>unsafe</code> functions or methods (including <code>extern</code> FFI functions)</li>
  <li>Implement <code>unsafe</code> traits (e.g., <code>Send</code>, <code>Sync</code>)</li>
  <li>Access or modify mutable static variables</li>
  <li>Access fields of <code>union</code> types</li>
</ol>
<h3>Send and Sync</h3>
<p>These are auto-traits: the compiler derives them automatically based on the types your type contains. <code>Send</code> means "safe to move to another thread". <code>Sync</code> means "<code>&amp;T</code> is safe to share across threads" (equivalently, <code>T: Sync ⟺ &amp;T: Send</code>). You almost never write <code>unsafe impl Send</code>.</p>
<p>Types that are <b>NOT</b> <code>Send</code>/<code>Sync</code>: <code>Rc&lt;T&gt;</code>, <code>Cell&lt;T&gt;</code>, <code>RefCell&lt;T&gt;</code>, raw pointers. The compiler will stop you from sending these across threads.</p>
<h3>When to reach for unsafe</h3>
<ul>
  <li>FFI: calling C libraries via <code>extern "C"</code></li>
  <li>Implementing safe abstractions (this is how <code>Vec</code>, <code>String</code>, <code>BTreeMap</code> are built)</li>
  <li>Performance hot loops where you can prove bounds checking is unnecessary</li>
</ul>"#,
            code: r#"// unsafe does NOT disable the borrow checker —
// it only adds 5 extra capabilities with a responsibility to uphold invariants.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // --- Superpower 1: dereference a raw pointer ---
    // This is safe to call here because `x` is live and aligned for the whole block.
    let x: i32 = 42;
    let raw: *const i32 = &x as *const i32;
    let value = unsafe { *raw }; // SAFE: x is live, aligned, initialised
    println!("Raw pointer deref: {}", value);

    // --- Superpower 2: call an unsafe fn ---
    // slice::get_unchecked is unsafe — caller must guarantee index is in bounds.
    let data = [10i32, 20, 30, 40, 50];
    let third = unsafe { data.get_unchecked(2) }; // SAFE: 2 < 5
    println!("get_unchecked(2) : {}", third);

    // --- fn pointers are always safe ---
    let fp: fn(i32, i32) -> i32 = add;
    println!("fn pointer result: {}", fp(3, 4));

    // --- Send / Sync auto-traits ---
    // These work because i32 is Send + Sync.
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    assert_send_sync(&x);
    println!("i32 is Send + Sync: confirmed at compile time");

    // --- What unsafe CANNOT do ---
    println!("\nunsafe superpowers (all 5):");
    println!("  1. Dereference raw pointers");
    println!("  2. Call unsafe functions / FFI");
    println!("  3. Implement unsafe traits (Send, Sync, GlobalAlloc, …)");
    println!("  4. Mutate static variables");
    println!("  5. Access union fields");
    println!("  (borrow checker and type system still fully enforced inside unsafe blocks)");
}"#,
            notes: vec![
                "unsafe has 5 extra capabilities; it does NOT suspend the borrow checker or type system.",
                "Send: safe to move across threads. Sync: &T is safe to share. Both are auto-derived from fields.",
                "Rc, RefCell, Cell, and raw pointers are not Send/Sync — the compiler blocks cross-thread use.",
                "FFI: every extern fn is unsafe to call because the compiler cannot verify C's contracts.",
                "Interview tip: 'what does unsafe do?' — the answer is the 5 superpowers list, not 'disables safety'.",
            ],
        },
    ]
}
