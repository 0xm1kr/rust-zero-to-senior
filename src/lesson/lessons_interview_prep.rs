use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "rust-interview-cheatsheet",
            category: "Interview Prep",
            title: "Senior Rust Interview Cheatsheet",
            description: r#"<p>A dense one-page cram sheet for senior Rust interviews. Know <em>why</em>, not just <em>what</em>.</p>

<h3>Ownership</h3>
<ul>
  <li>Every value has exactly <b>one owner</b>. When the owner goes out of scope, the value is <b>dropped</b> (RAII — no GC needed).</li>
  <li>Ownership can be <b>moved</b> (transferred) or <b>borrowed</b> (lent as a reference).</li>
</ul>

<h3>Borrow checker rules</h3>
<ul>
  <li>Any number of <b>shared references</b> (<code>&amp;T</code>) OR exactly <b>one mutable reference</b> (<code>&amp;mut T</code>) — never both simultaneously.</li>
  <li>References must not <b>outlive</b> the data they point to (no dangling pointers, guaranteed at compile time).</li>
</ul>

<h3>Lifetime elision — the 3 rules</h3>
<ol>
  <li>Each input reference parameter gets its own distinct lifetime.</li>
  <li>If there is exactly one input lifetime, it is assigned to all output lifetimes.</li>
  <li>If one of the input lifetimes is <code>&amp;self</code> or <code>&amp;mut self</code>, its lifetime is assigned to all output lifetimes.</li>
</ol>

<h3>Copy vs Clone vs Move</h3>
<ul>
  <li><b>Copy</b>: implicit bitwise copy on assignment — <code>i32</code>, <code>bool</code>, <code>char</code>, <code>f64</code>, <code>(A, B)</code> where A, B: Copy, <code>&amp;T</code>. No heap involvement.</li>
  <li><b>Clone</b>: explicit deep copy via <code>.clone()</code>. May allocate. Must be called manually.</li>
  <li><b>Move</b>: the default for non-Copy types. Transfers ownership; the original is inaccessible afterwards.</li>
</ul>

<h3>Send + Sync</h3>
<ul>
  <li><code>Send</code>: safe to <b>move</b> to another thread.</li>
  <li><code>Sync</code>: <code>&amp;T</code> is safe to <b>share</b> across threads (equivalently, <code>T: Sync ⟺ &amp;T: Send</code>).</li>
  <li>Both are <b>auto-traits</b>: derived automatically unless a field is not Send/Sync (<code>Rc</code>, <code>RefCell</code>, raw pointers, …).</li>
</ul>

<h3>Smart pointers</h3>
<ul>
  <li><code>Box&lt;T&gt;</code>: single owner, heap allocation. Use for large values, recursive types, <code>dyn Trait</code>.</li>
  <li><code>Rc&lt;T&gt;</code>: reference-counted, single thread. Use when multiple owners need to share data.</li>
  <li><code>Arc&lt;T&gt;</code>: atomically reference-counted, multi-thread. Same idea, thread-safe.</li>
</ul>

<h3>Interior mutability</h3>
<ul>
  <li><code>Cell&lt;T&gt;</code>: Copy types only. get()/set(). No runtime cost. Never panics.</li>
  <li><code>RefCell&lt;T&gt;</code>: any type. Runtime borrow check. Panics on violation. Single thread.</li>
  <li><code>Mutex&lt;T&gt;</code>: like RefCell but thread-safe. Blocks on contention.</li>
  <li>Common combo: <code>Rc&lt;RefCell&lt;T&gt;&gt;</code> (single thread) / <code>Arc&lt;Mutex&lt;T&gt;&gt;</code> (multi-thread).</li>
</ul>

<h3>Common stdlib collections</h3>
<ul>
  <li><code>Vec&lt;T&gt;</code>: dynamic array. O(1) push/pop back.</li>
  <li><code>VecDeque&lt;T&gt;</code>: double-ended queue. O(1) push/pop both ends.</li>
  <li><code>HashMap&lt;K, V&gt;</code>: hash map. O(1) avg get/insert.</li>
  <li><code>BTreeMap&lt;K, V&gt;</code>: sorted map. O(log n) get/insert.</li>
  <li><code>HashSet&lt;T&gt;</code> / <code>BTreeSet&lt;T&gt;</code>: set variants of the above.</li>
  <li><code>BinaryHeap&lt;T&gt;</code>: max-heap. Wrap in <code>Reverse(x)</code> for min-heap.</li>
</ul>

<h3>Error handling</h3>
<ul>
  <li><code>Result&lt;T, E&gt;</code>: explicit success/failure. No exceptions.</li>
  <li><code>?</code> operator: unwraps <code>Ok</code> or early-returns <code>Err</code>, using <code>From</code> to convert error types.</li>
  <li><code>thiserror</code>: derive-macro for library error types.</li>
  <li><code>anyhow</code>: flexible boxed error for applications.</li>
</ul>

<h3>Async / await mental model</h3>
<ul>
  <li><code>async fn</code> returns a <code>Future</code> — a state machine. Nothing runs until polled.</li>
  <li><code>.await</code> suspends the current task if the future is not ready; the runtime polls other tasks.</li>
  <li>Requires a runtime (<code>tokio</code>, <code>async-std</code>). The runtime provides the executor and I/O reactor.</li>
  <li>Key: futures are <b>lazy</b> and <b>cancel-safe by dropping</b> (usually — check per-type).</li>
</ul>

<h3>Iterators — zero-cost story</h3>
<ul>
  <li>Adapters (<code>map</code>, <code>filter</code>, <code>flat_map</code>, <code>take</code>, …) compile to the same asm as hand-written loops via monomorphization + inlining.</li>
  <li><code>.collect::&lt;Vec&lt;_&gt;&gt;()</code> is the <em>only</em> common adapter that heap-allocates.</li>
</ul>

<h3>unsafe contract</h3>
<ul>
  <li>Five superpowers only: raw pointer deref, calling unsafe fns, implementing unsafe traits, union field access, mutating statics.</li>
  <li>The borrow checker and type system <b>still apply</b> inside <code>unsafe</code> blocks.</li>
  <li>Your responsibility: uphold memory-safety invariants the compiler cannot verify.</li>
</ul>

<h3>Common derive macros</h3>
<p><code>Debug</code>, <code>Clone</code>, <code>Copy</code>, <code>PartialEq</code>, <code>Eq</code>, <code>PartialOrd</code>, <code>Ord</code>, <code>Hash</code>, <code>Default</code>, <code>Serialize</code>, <code>Deserialize</code>.</p>

<h3>Top pitfalls</h3>
<ul>
  <li>Iterating + mutating the same collection (fix: collect indices first).</li>
  <li>Dangling reference returned from a function (fix: return owned, or borrow the parameter).</li>
  <li>Large <code>.clone()</code> in a hot path (fix: pass references, use Cow).</li>
  <li>Premature <code>.unwrap()</code> in production code (fix: propagate with <code>?</code>, handle <code>None</code>/<code>Err</code>).</li>
  <li>Integer overflow on <code>usize</code> indices (Rust panics in debug; use checked/saturating arithmetic).</li>
</ul>

<h3>Study list</h3>
<ul>
  <li><a href="https://doc.rust-lang.org/book/" target="_blank">The Rust Programming Language</a> (the Book) — chapters 4, 10, 16, 20</li>
  <li><a href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a> — quick reference with runnable snippets</li>
  <li><a href="https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa" target="_blank">Crust of Rust</a> — Jon Gjengset's YouTube series (lifetimes, iterators, smart pointers in depth)</li>
  <li><a href="https://doc.rust-lang.org/nomicon/" target="_blank">The Rustonomicon</a> — unsafe Rust deep dive</li>
</ul>"#,
            code: r#"fn main() {
    println!("Hello, future senior Rustacean!\n");

    // A quick self-check — can you answer each of these without hesitation?
    let topics = [
        // Ownership & borrowing
        "Ownership: one owner, dropped at end of scope (RAII, no GC)",
        "Borrowing: N shared OR 1 exclusive — never both simultaneously",
        "Lifetimes: annotations express how reference scopes relate",
        "Elision rule 1: each input ref gets its own lifetime",
        "Elision rule 2: one input lifetime → all outputs get it",
        "Elision rule 3: &self lifetime → all output lifetimes",
        // Types
        "Copy: implicit bitwise clone (stack types: i32, bool, char, &T)",
        "Clone: explicit deep copy — may allocate, always intentional",
        "Move: default for non-Copy — original inaccessible after",
        // Threading
        "Send: safe to move across threads",
        "Sync: &T safe to share across threads (Sync ⟺ &T: Send)",
        "Rc/RefCell are NOT Send/Sync — compile error if you try",
        // Heap
        "Box<T>: single-owner heap. Rc<T>: shared single-thread. Arc<T>: shared multi-thread",
        "Cell<T>: Copy only, no borrows, never panics",
        "RefCell<T>: runtime borrow check, panics on violation",
        "Mutex<T>: thread-safe interior mutability, blocks on contention",
        // Errors
        "Result<T,E> + ? + From: the complete error propagation system",
        "thiserror: library errors. anyhow: application errors",
        // Async
        "async fn returns a lazy Future (state machine, zero-cost)",
        ".await suspends task; runtime polls others; requires tokio/async-std",
        // Performance
        "Iterator adapters: zero-cost via monomorphization + inlining",
        "collect::<Vec<_>>() is the only common allocating adapter",
        "Vec::with_capacity(n): pre-allocate to avoid O(log n) reallocations",
        // Unsafe
        "unsafe: 5 superpowers only — borrow checker still fully active inside",
        // Pitfalls
        "Pitfall: iterate + mutate → collect indices first, then mutate",
        "Pitfall: large .clone() in hot path → pass references or use Cow",
        "Pitfall: premature .unwrap() → propagate with ? or handle explicitly",
    ];

    for (i, topic) in topics.iter().enumerate() {
        println!("  {:2}. {}", i + 1, topic);
    }

    println!("\nStudy resources:");
    println!("  • The Rust Book (doc.rust-lang.org/book) — chapters 4, 10, 16, 20");
    println!("  • Rust by Example (doc.rust-lang.org/rust-by-example)");
    println!("  • Crust of Rust — Jon Gjengset on YouTube");
    println!("  • The Rustonomicon (doc.rust-lang.org/nomicon) — for unsafe deep-dive");
}"#,
            notes: vec![
                "Ownership + borrowing + lifetimes is the core trifecta. Be able to explain the rules, not just recite them.",
                "Lifetime elision: the 3 rules cover ~95% of cases. Know them cold — interviewers love to test this.",
                "Send/Sync are compiler-enforced at the type level. You almost never write 'unsafe impl Send'.",
                "Error handling story: Result → ? → From → thiserror (library) / anyhow (app). No exceptions in Rust.",
                "Zero-cost iterators: the punchline is monomorphization. 'The compiler turns the chain into a single loop.'",
                "For async: 'Future is a lazy state machine; .await suspends; tokio is the executor that calls poll().'",
                "unsafe contract: 5 superpowers, borrow checker still active, you manually uphold the invariants.",
            ],
        },
    ]
}
