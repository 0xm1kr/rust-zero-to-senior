use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "ownership-basics",
            category: "Ownership & Borrowing",
            title: "Ownership Rules",
            description: r##"
<p>Ownership is Rust's most distinctive feature — it replaces the garbage collector with a set of
compile-time rules that guarantee memory safety at zero runtime cost.</p>

<h3>The Three Rules</h3>
<ul>
  <li>Every value has exactly <b>one owner</b> (a variable binding).</li>
  <li>There can only be <b>one owner at a time</b>.</li>
  <li>When the owner goes <b>out of scope</b>, the value is <b>dropped</b> (memory freed, destructors run — RAII).</li>
</ul>

<h3>Move Semantics</h3>
<p>Assigning a non-<code>Copy</code> value to another binding <b>moves</b> it. The original binding is
invalidated by the compiler — not at runtime, at compile time:</p>
<pre>
let s = String::from("hi");
let t = s;           // s is MOVED into t
println!("{}", s);   // ❌ compile error: use of moved value
</pre>
<p>This is the #1 surprise for engineers coming from TypeScript, where assignment just copies a reference.
In Rust, <code>String</code> is heap data — there is no implicit shared reference.</p>

<h3>How to Fix a Move</h3>
<ul>
  <li><b>Clone</b> — explicit deep copy: <code>let t = s.clone();</code> — both bindings valid, two heap allocations.</li>
  <li><b>Borrow</b> — pass a reference: <code>let t = &amp;s;</code> — <code>s</code> stays the owner, <code>t</code> borrows it.</li>
</ul>

<h3>Copy Types</h3>
<p>Primitive types (<code>i32</code>, <code>u64</code>, <code>bool</code>, <code>char</code>, <code>f64</code>, pointers, and fixed-size tuples of
Copy types) implement the <code>Copy</code> trait. Assignment <b>duplicates</b> the bits on the stack — no move,
no invalidation. Stack-only data is cheap to copy; heap data is not, so heap types are never <code>Copy</code>.</p>
"##,
            code: r##"fn main() {
    // ── Move semantics ──────────────────────────────────────────────────
    let s = String::from("hello");
    let t = s;              // s is MOVED into t; s is invalidated
    // println!("{}", s);   // ← uncommenting this is a compile error
    println!("t owns the string: {t}");

    // Fix 1: clone — explicit deep copy
    let a = String::from("world");
    let b = a.clone();      // two independent heap buffers
    println!("a={a}, b={b}");

    // Fix 2: borrow — a reference, not a move
    let c = String::from("rust");
    let d = &c;             // d borrows c; c is still the owner
    println!("c={c}, d borrows it as {d}");

    // ── Copy types — no move, just bit-copy ─────────────────────────────
    let x: i32 = 42;
    let y = x;              // x is NOT moved; i32 is Copy
    println!("x={x}, y={y}");  // both valid

    let flag = true;
    let flag2 = flag;
    println!("flag={flag}, flag2={flag2}");

    // ── RAII: drop happens when scope ends ──────────────────────────────
    {
        let temp = String::from("I live briefly");
        println!("inside scope: {temp}");
    }   // ← temp is dropped here; heap memory freed automatically
    println!("temp is gone; no dangling pointer possible");
}
"##,
            notes: vec![
                "Each value has exactly one owner; ownership is transferred (moved) on assignment for non-Copy types.",
                "When an owner goes out of scope, Rust calls drop() automatically — no GC, no manual free().",
                "Copy types (primitives) are duplicated on assignment; non-Copy types (String, Vec) are moved.",
                "Fix a move with .clone() for a deep copy, or & to borrow without transferring ownership.",
                "The compiler enforces all ownership rules at compile time — no runtime cost.",
            ],
        },
        Lesson {
            id: "borrowing",
            category: "Ownership & Borrowing",
            title: "References & Borrowing",
            description: r##"
<p>A <b>reference</b> lets you access a value without taking ownership of it. Rust has two kinds:</p>
<ul>
  <li><code>&amp;T</code> — immutable (shared) borrow. Many can exist simultaneously.</li>
  <li><code>&amp;mut T</code> — mutable (exclusive) borrow. Only <b>one</b> may exist; no shared borrows at the same time.</li>
</ul>

<h3>The Cardinal Rule</h3>
<p>At any given point in code: <b>many <code>&amp;T</code> OR exactly one <code>&amp;mut T</code> — never both</b>.
This rule eliminates data races at compile time, making Rust's concurrency story provably safe.</p>

<h3>TypeScript Comparison</h3>
<p>Think of <code>&amp;mut</code> like acquiring an exclusive lock on an object — while you hold it, no one
else can read or write. Rust enforces this statically; TypeScript doesn't enforce it at all.</p>

<h3>No Dangling References</h3>
<p>The compiler guarantees a reference <b>always points to valid memory</b>. You cannot return a reference
to a local variable (it would dangle after the function returns). Rust calls this the
<b>borrow checker</b> — it tracks lifetimes to ensure safety. Lifetime annotations (covered in their
own lesson) are how you teach the borrow checker about longer-lived references.</p>

<h3>Function Signatures</h3>
<p>Prefer <code>&amp;T</code> for read-only parameters and <code>&amp;mut T</code> when the function needs to mutate.
This is far cheaper than cloning: no heap allocation, no copy.</p>
"##,
            code: r##"fn print_len(s: &str) {
    // Immutable borrow — read-only access, zero cost
    println!("  length: {}", s.len());
}

fn make_excited(s: &mut String) {
    // Mutable borrow — exclusive access
    s.push_str("!!!");
}

fn loudest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // Returning a reference requires lifetime annotation
    // so the compiler knows which input the output borrows from.
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    // ── Many immutable borrows are fine simultaneously ───────────────────
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1={r1}, r2={r2}, original={s}"); // all valid

    // ── Mutable borrow requires exclusive access ─────────────────────────
    let mut t = String::from("world");
    make_excited(&mut t);
    println!("after mutation: {t}");
    print_len(&t);          // immutable borrow works again after &mut ends

    // ── The borrow checker prevents use-after-free ──────────────────────
    // This would NOT compile — mutable and immutable borrows overlap:
    //   let r_imm = &t;
    //   let r_mut = &mut t;  // error: cannot borrow as mutable
    //   println!("{r_imm}");

    // ── Returning a reference with lifetime annotation ───────────────────
    let word1 = String::from("cat");
    let word2 = String::from("elephant");
    let winner = loudest(&word1, &word2);
    println!("loudest: {winner}");

    // ── Function parameters accept references, not owned values ──────────
    let greeting = String::from("Rust");
    print_len(&greeting);   // &String coerces to &str automatically
    print_len("literal");   // &str literal also works
    println!("greeting still owned: {greeting}");
}
"##,
            notes: vec![
                "& is an immutable (shared) borrow; &mut is a mutable (exclusive) borrow.",
                "The cardinal rule: many &T OR one &mut T, never both at the same time.",
                "References are always valid — the borrow checker prevents dangling pointers at compile time.",
                "Functions should take &T or &mut T instead of T to avoid unnecessary moves or clones.",
                "Returning a reference from a function requires lifetime annotations to satisfy the borrow checker.",
            ],
        },
        Lesson {
            id: "move-vs-copy",
            category: "Ownership & Borrowing",
            title: "Move vs Copy vs Clone",
            description: r##"
<p>Understanding how Rust transfers values is critical for writing ergonomic APIs and avoiding
accidental clone overhead.</p>

<h3>Copy — stack-only, implicit duplicate</h3>
<p>Types that fit entirely on the stack and are cheap to duplicate implement <code>Copy</code>:
<code>i8</code>–<code>i128</code>, <code>u8</code>–<code>u128</code>, <code>f32</code>, <code>f64</code>, <code>bool</code>, <code>char</code>, raw pointers,
and tuples/arrays <em>where every element is also <code>Copy</code></em>. Assignment silently copies the bits.
There is <b>no</b> implicit <code>Copy</code> for anything that owns heap memory.</p>

<h3>Move — the default for non-Copy types</h3>
<p><code>String</code>, <code>Vec&lt;T&gt;</code>, <code>Box&lt;T&gt;</code>, and virtually all custom structs move on assignment.
The old binding becomes inaccessible to the compiler — this is enforced statically, not at runtime.</p>

<h3>Clone — explicit deep copy</h3>
<p><code>.clone()</code> creates a brand-new heap allocation. It is <b>never</b> implicit — you always see it in
the source code. Cloning a <code>Vec&lt;i64&gt;</code> with a million elements copies a million <code>i64</code> values;
that is real work. Design APIs to accept <code>&amp;[T]</code> or <code>&amp;T</code> when you only need to read.</p>

<h3>TypeScript Comparison</h3>
<p>In TypeScript, <code>const b = a</code> copies the reference for objects (shallow). In Rust,
<code>let b = a</code> either moves (heap types) or copies bits (stack types). There is no implicit
reference sharing — that is the whole point.</p>

<h3>Deriving Copy and Clone</h3>
<p>You can <code>#[derive(Copy, Clone)]</code> on structs where all fields are <code>Copy</code>. You can
<code>#[derive(Clone)]</code> alone for structs with heap fields.</p>
"##,
            code: r##"fn sum_slice(data: &[i64]) -> i64 {
    // Takes a reference — no clone needed regardless of Vec size
    data.iter().sum()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // ── Copy types: assignment duplicates, both bindings valid ───────────
    let a: i32 = 42;
    let b = a;          // bit-copy; a is NOT invalidated
    println!("a={a}, b={b}");

    let flag = true;
    let f2 = flag;
    println!("flag={flag}, f2={f2}");

    // Custom Copy struct
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;        // copied, not moved (both fields are f64 = Copy)
    println!("p1={p1:?}, p2={p2:?}");

    // ── Move: String is NOT Copy ─────────────────────────────────────────
    let s = String::from("hello");
    let t = s;          // s is moved; s is now invalid
    println!("t owns the string: {t}");
    // println!("{s}"); // ← compile error

    // ── Clone: explicit deep copy ────────────────────────────────────────
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2 = v1.clone();    // separate heap buffer — real allocation
    println!("v1={v1:?}");
    println!("v2={v2:?}");

    // ── Prefer & to avoid cloning in function calls ──────────────────────
    let big: Vec<i64> = (0_i64..100_000).collect();
    println!("sum (no clone): {}", sum_slice(&big));
    println!("big still owned, len={}", big.len());
}
"##,
            notes: vec![
                "Copy types (primitives, small stack types) are duplicated on assignment — no move.",
                "Non-Copy types (String, Vec, Box) are moved on assignment; the old binding is invalidated.",
                ".clone() creates an explicit deep copy — it is never implicit in Rust.",
                "Cloning heap data is real work; prefer passing &T or &[T] when you only need to read.",
                "Derive #[derive(Copy, Clone)] on structs only when all fields are themselves Copy.",
            ],
        },
        Lesson {
            id: "slices",
            category: "Ownership & Borrowing",
            title: "Slices: &str and &[T]",
            description: r##"
<p>A <b>slice</b> is a <em>fat pointer</em> — a pointer to some location in an existing buffer plus a
length. It does not own any data; it borrows a contiguous subsequence from something that does.</p>

<h3>&amp;str — string slice</h3>
<p><code>&amp;str</code> is a borrowed view into a <code>String</code>'s heap buffer or into a string literal embedded in
the binary. It is the <em>preferred type for read-only string parameters</em> because it accepts both
<code>&amp;String</code> (via automatic coercion) and <code>&amp;"literal"</code> without any allocation.</p>
<pre>
fn greet(name: &amp;str) { ... }

greet("Alice");                        // &amp;'static str — lives in binary
greet(&amp;String::from("Bob"));          // &amp;String coerces to &amp;str
</pre>

<h3>&amp;[T] — slice of any type</h3>
<p><code>&amp;[T]</code> is a borrowed view into a <code>Vec&lt;T&gt;</code> or an array <code>[T; N]</code>. Functions that accept
<code>&amp;[T]</code> work with either source, making them more flexible than accepting <code>&amp;Vec&lt;T&gt;</code>.</p>

<h3>Indexing and range syntax</h3>
<p><code>&amp;s[start..end]</code> — start inclusive, end exclusive. <code>&amp;s[..3]</code>, <code>&amp;s[2..]</code>, <code>&amp;s[..]</code> also work.
<b>Warning:</b> <code>&amp;str</code> is <em>byte-indexed</em>, not char-indexed. Slicing mid-character panics at
runtime. Use <code>.chars()</code> iteration when dealing with non-ASCII text.</p>

<h3>TypeScript Comparison</h3>
<p>TypeScript's <code>substring()</code> / <code>slice()</code> always allocates a new string. Rust's <code>&amp;s[..]</code>
creates a zero-allocation view — no heap allocation, no copy, just a pointer and a length.</p>
"##,
            code: r##"fn count_vowels(s: &str) -> usize {
    // Works for &str literals AND &String — same function, no overload
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

fn sum_slice(nums: &[i32]) -> i32 {
    // Works for Vec<i32> AND [i32; N] arrays
    nums.iter().sum()
}

fn first_word(s: &str) -> &str {
    // Returns a slice of the input — no allocation
    match s.find(' ') {
        Some(i) => &s[..i],
        None    => s,
    }
}

fn main() {
    // ── &str slices ──────────────────────────────────────────────────────
    let owned = String::from("Hello, Rust!");
    let hello: &str = &owned[0..5];  // fat pointer into owned's buffer
    println!("slice: '{hello}'");

    let sentence = String::from("the quick brown fox");
    println!("first word: '{}'", first_word(&sentence));

    // &str literal — pointer into the binary, 'static lifetime
    let lit: &str = "I live in the binary";
    println!("vowels in literal: {}", count_vowels(lit));
    println!("vowels in owned:   {}", count_vowels(&owned)); // coercion

    // ── &[T] slices ──────────────────────────────────────────────────────
    let v = vec![10, 20, 30, 40, 50];
    let mid: &[i32] = &v[1..4];   // [20, 30, 40] — no allocation
    println!("mid slice: {mid:?}");
    println!("sum of all: {}", sum_slice(&v));

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("sum of array: {}", sum_slice(&arr)); // array coerces to &[T]

    // ── Byte-indexing caution ────────────────────────────────────────────
    let accented = "héllo";
    println!("byte length: {} (not 5!)", accented.len()); // 6
    println!("char count: {}",  accented.chars().count()); // 5
    // &accented[1..2] would PANIC — 'é' occupies bytes 1 and 2
    // Safe: iterate chars instead
    let chars: Vec<char> = accented.chars().collect();
    println!("second char: '{}'", chars[1]);
}
"##,
            notes: vec![
                "A slice (&str, &[T]) is a fat pointer (ptr + length) — zero allocation, zero copy.",
                "&str is the preferred parameter type for strings; &String coerces to &str automatically.",
                "&[T] is the preferred parameter type for sequences; Vec<T> and arrays both coerce to &[T].",
                "&str indexing is byte-based, not char-based — slicing mid-UTF-8 character panics at runtime.",
                "Returning a &str or &[T] from a function returns a view into the input — lifetime ensures it stays valid.",
            ],
        },
        Lesson {
            id: "string-vs-str",
            category: "Ownership & Borrowing",
            title: "String vs &str",
            description: r##"
<p>The <code>String</code> / <code>&amp;str</code> split is confusing at first but maps to a clean mental model once
you see it through the ownership lens.</p>

<h3>String — owned, heap-allocated, growable</h3>
<p><code>String</code> is essentially a <code>Vec&lt;u8&gt;</code> that is guaranteed to contain valid UTF-8. It owns
its buffer; it can grow, shrink, and be mutated. Use it when you need to <b>build, store, or own</b>
a string value — in a struct field, as a function return type, in a collection.</p>

<h3>&amp;str — borrowed, immutable view</h3>
<p><code>&amp;str</code> is a slice: a pointer + length into someone else's UTF-8 buffer. It cannot grow.
Use it for <b>read-only parameters</b> — this is the idiomatic Rust function signature:
<code>fn process(name: &amp;str)</code>. Both <code>&amp;String</code> and string literals coerce to <code>&amp;str</code> automatically.</p>

<h3>Conversions</h3>
<ul>
  <li><code>&amp;str → String</code>: <code>.to_string()</code>, <code>.to_owned()</code>, <code>String::from(s)</code>, <code>format!("{s}")</code></li>
  <li><code>&amp;String → &amp;str</code>: automatic coercion, or explicit <code>s.as_str()</code></li>
</ul>

<h3>Concatenation</h3>
<p><code>s1 + &amp;s2</code> uses the <code>Add</code> trait — it consumes <code>s1</code> (moves it) and appends <code>s2</code>.
<code>format!("{s1}{s2}")</code> borrows both and returns a fresh <code>String</code> — safer and clearer for
multiple operands.</p>

<h3>UTF-8 and length</h3>
<p><code>.len()</code> returns <b>bytes</b>; <code>.chars().count()</code> returns Unicode scalar values.
<code>"héllo".len() == 6</code> (not 5) because <code>é</code> is 2 bytes. Always use <code>.chars()</code>
when you care about characters, not bytes.</p>
"##,
            code: r##"fn greet(name: &str) {
    // Canonical: accept &str, not &String
    // Works with literals, &String, and String::as_str()
    println!("Hello, {name}!");
}

fn build_greeting(name: &str, title: &str) -> String {
    // Return owned String when the caller needs to keep the value
    format!("Dear {title} {name},")
}

fn main() {
    // ── &str literals ────────────────────────────────────────────────────
    let lit: &str = "I am in the binary";
    println!("{lit}");

    // ── String: owned, heap-allocated, mutable ───────────────────────────
    let mut owned = String::from("Rust");
    owned.push_str(" is fast");
    owned.push('!');
    println!("{owned}");

    // ── Conversions from &str to String ─────────────────────────────────
    let s1: String = lit.to_string();
    let s2: String = lit.to_owned();
    let s3: String = String::from(lit);
    let s4: String = format!("copy: {lit}");
    println!("{s1} | {s2} | {s3} | {s4}");

    // ── Concatenation ───────────────────────────────────────────────────
    let first  = String::from("Hello");
    let second = String::from(", world");
    let joined = first + &second;   // first is MOVED; second is borrowed
    println!("{joined}");

    let a = String::from("foo");
    let b = String::from("bar");
    let c = format!("{a}{b}");      // borrows both, no moves
    println!("{c} — a={a}, b={b}"); // a and b still valid

    // ── Function accepts &str — coercion is automatic ────────────────────
    greet("world");                 // &str literal
    greet(&owned);                  // &String coerces to &str
    greet(owned.as_str());          // explicit, same result

    let msg = build_greeting("Smith", "Dr.");
    println!("{msg}");

    // ── UTF-8: bytes vs chars ────────────────────────────────────────────
    let emoji = String::from("héllo");
    println!("bytes={}, chars={}", emoji.len(), emoji.chars().count()); // 6, 5
}
"##,
            notes: vec![
                "String is an owned, heap-allocated, growable UTF-8 buffer — use for struct fields and return types.",
                "&str is a borrowed, immutable view into a UTF-8 buffer — the preferred read-only parameter type.",
                "Both &String and &str literals coerce to &str; you almost never need to accept &String.",
                "format!() is the safest concatenation: it borrows everything and returns a new String.",
                ".len() returns bytes; .chars().count() returns Unicode scalar values — 'héllo'.len() == 6.",
            ],
        },
        Lesson {
            id: "smart-pointers",
            category: "Ownership & Borrowing",
            title: "Box, Rc, Arc — Heap & Shared Ownership",
            description: r##"
<p>Rust's ownership model is single-owner by default, but three smart pointers cover the cases where
you need heap allocation, shared ownership, or interior mutability.</p>

<h3>Box&lt;T&gt; — heap allocation, single owner</h3>
<p>Allocates <code>T</code> on the heap. The <code>Box</code> itself is a thin pointer on the stack. Use it for:</p>
<ul>
  <li><b>Recursive types</b> — <code>enum List { Cons(i32, Box&lt;List&gt;), Nil }</code>. Without <code>Box</code>, the compiler
    cannot compute the size.</li>
  <li><b>Trait objects</b> — <code>Box&lt;dyn Trait&gt;</code> for dynamic dispatch.</li>
  <li><b>Large values</b> — avoid stack overflow by boxing big structs.</li>
</ul>

<h3>Rc&lt;T&gt; — shared ownership, single thread</h3>
<p>Reference-counted pointer. <code>Rc::clone(&amp;ptr)</code> increments the count; when the last clone is
dropped, the value is freed. Not <code>Send</code> — cannot cross thread boundaries.
Pair with <code>RefCell&lt;T&gt;</code> for <b>interior mutability</b>: <code>Rc&lt;RefCell&lt;T&gt;&gt;</code> lets multiple
owners mutate the value at runtime (borrow checked dynamically instead of statically).</p>

<h3>Arc&lt;T&gt; — shared ownership, thread-safe</h3>
<p><code>Arc</code> ("Atomically Reference Counted") is <code>Rc</code> with an atomic counter — safe to clone
and share across threads. Pair with <code>Mutex&lt;T&gt;</code> or <code>RwLock&lt;T&gt;</code> for shared mutable state:
<code>Arc&lt;Mutex&lt;T&gt;&gt;</code>. The Concurrency lesson goes deep on this pattern.</p>

<h3>Choosing</h3>
<p>Single owner, heap? → <code>Box</code>. Multiple owners, one thread? → <code>Rc</code>. Multiple owners,
many threads? → <code>Arc</code>. Need mutation through shared reference? Add <code>RefCell</code> or <code>Mutex</code>.</p>
"##,
            code: r##"use std::rc::Rc;
use std::cell::RefCell;

// Box enables recursive enum types (size is otherwise unknowable)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // ── Box<T>: heap allocation, single owner ────────────────────────────
    let boxed: Box<i32> = Box::new(99);
    println!("boxed value: {boxed}"); // auto-deref

    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    println!("linked list: {list:?}");

    // ── Rc<T>: shared ownership, single thread ───────────────────────────
    let shared = Rc::new(String::from("shared data"));
    let clone1 = Rc::clone(&shared); // increments reference count
    let clone2 = Rc::clone(&shared);
    println!("strong_count = {}", Rc::strong_count(&shared)); // 3
    println!("all see: {shared} | {clone1} | {clone2}");

    drop(clone2);
    println!("after drop: strong_count = {}", Rc::strong_count(&shared)); // 2

    // ── Rc<RefCell<T>>: shared + interior mutability (single thread) ─────
    let counter = Rc::new(RefCell::new(0_i32));
    let c1 = Rc::clone(&counter);
    let c2 = Rc::clone(&counter);

    *c1.borrow_mut() += 10; // borrow checked at runtime
    *c2.borrow_mut() += 5;
    println!("counter = {}", counter.borrow()); // 15

    // ── Arc<T>: thread-safe shared ownership ─────────────────────────────
    // (full Arc<Mutex<T>> example lives in the Concurrency lesson)
    use std::sync::Arc;
    let arc_val = Arc::new(42_u64);
    let arc2 = Arc::clone(&arc_val);
    println!("arc value: {arc_val}, clone: {arc2}");
    println!("Use Arc<Mutex<T>> for mutable shared state across threads.");
}
"##,
            notes: vec![
                "Box<T> heap-allocates T with a single owner — required for recursive types and dyn Trait objects.",
                "Rc<T> enables multiple owners in single-threaded code via reference counting; not Send.",
                "Arc<T> is the thread-safe Rc — use it with Mutex<T> or RwLock<T> for shared mutable state.",
                "Rc<RefCell<T>> gives shared mutability in single-threaded code; borrow rules checked at runtime.",
                "Prefer single ownership (&T / &mut T) by default; reach for Rc/Arc only when ownership is genuinely shared.",
            ],
        },
    ]
}
