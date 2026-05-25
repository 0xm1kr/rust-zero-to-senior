use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "stack-vs-heap",
            category: "Memory & Performance",
            title: "Stack vs Heap, Owned vs Borrowed",
            description: r#"<p><b>Stack</b>: fixed-size types allocated in LIFO order — <code>i32</code>, <code>bool</code>, structs of stack types, <code>[T; N]</code>. Essentially free: the stack pointer just moves a few bytes.</p>
<p><b>Heap</b>: dynamically-sized data. <code>String</code>, <code>Vec&lt;T&gt;</code>, <code>Box&lt;T&gt;</code> heap-allocate their payload. The <em>handle</em> (pointer + metadata) lives on the stack.</p>
<p><b>Borrowed views</b>: <code>&amp;str</code> is a fat pointer (ptr + length) into some other storage — a string literal in <code>.rodata</code>, or the heap buffer of a <code>String</code>. It owns nothing and costs one stack word to pass.</p>
<h3>Size breakdown on 64-bit</h3>
<ul>
  <li><code>i32</code>: 4 bytes (pure stack)</li>
  <li><code>&amp;str</code>: 16 bytes (ptr + len — two words on the stack)</li>
  <li><code>String</code>: 24 bytes (ptr + len + capacity)</li>
  <li><code>Box&lt;i32&gt;</code>: 8 bytes (one pointer — the i32 is on the heap)</li>
  <li><code>Vec&lt;T&gt;</code>: 24 bytes (ptr + len + capacity, same layout as String)</li>
</ul>
<p><b>Rule of thumb</b>: prefer <code>&amp;str</code> / <code>&amp;[T]</code> in function signatures. Only take <code>String</code> / <code>Vec</code> when you need to store or transfer ownership. Small <code>Copy</code> types pass by value for free.</p>"#,
            code: r#"fn main() {
    use std::mem::size_of;

    println!("=== Size of types (bytes) ===");
    println!("i32           : {}", size_of::<i32>());       // 4
    println!("f64           : {}", size_of::<f64>());       // 8
    println!("bool          : {}", size_of::<bool>());      // 1
    println!("char          : {}", size_of::<char>());      // 4 (Unicode scalar)
    println!("&str (fat ptr): {}", size_of::<&str>());      // 16: ptr + len
    println!("String        : {}", size_of::<String>());    // 24: ptr + len + cap
    println!("Box<i32>      : {}", size_of::<Box<i32>>());  // 8: one pointer
    println!("Vec<u8>       : {}", size_of::<Vec<u8>>());   // 24: ptr + len + cap

    // &str borrows from a string literal stored in .rodata (not the heap)
    let literal: &str = "hello";

    // String owns heap memory; .to_owned() allocates
    let owned: String = literal.to_owned();

    // A second &str borrowing into owned's heap buffer — zero allocation
    let slice: &str = &owned[1..3];

    println!("\nliteral = {:?}  (points into read-only data)", literal);
    println!("owned   = {:?}  (heap-allocated buffer)", owned);
    println!("slice   = {:?}  (points into owned's buffer)", slice);

    // Box<T> moves T to the heap; the Box handle is a pointer on the stack
    let boxed: Box<i32> = Box::new(42);
    println!("\nBox<i32> value  : {}", boxed);
    println!("Box<i32> size   : {} bytes (just a pointer)", size_of::<Box<i32>>());

    // Accepting &str avoids cloning — both calls are zero-allocation
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    let s = String::from("Alice");
    greet(&s);      // String coerces to &str
    greet("Bob");   // literal &str — no allocation ever
}"#,
            notes: vec![
                "Copy types (i32, bool, f64, char, fixed arrays of Copy) clone implicitly with no heap involvement.",
                "&str is a fat pointer: 2 words on the stack. Passing by value is cheap — just copies the two words.",
                "String = Vec<u8> with a UTF-8 guarantee. Its three-word header (ptr/len/cap) lives on the stack.",
                "Box<T> is the simplest heap allocation. Dropped automatically at end of scope (RAII).",
                "Interview tip: know why &str is 16 bytes and String is 24 bytes — it comes up in every ownership discussion.",
            ],
        },
        Lesson {
            id: "iterators-zero-cost",
            category: "Memory & Performance",
            title: "Zero-Cost Iterators",
            description: r#"<p>Rust iterator adapters — <code>.filter()</code>, <code>.map()</code>, <code>.flat_map()</code>, <code>.take()</code>, etc. — are <b>zero-cost abstractions</b>. The compiler monomorphizes and inlines closures, producing machine code identical to a hand-written loop.</p>
<h3>Why it works</h3>
<ul>
  <li><b>Monomorphization</b>: generics produce one concrete type per instantiation — no dynamic dispatch, no virtual calls.</li>
  <li><b>Inlining</b>: closures become inline code; LLVM can eliminate bounds checks and fuse loops into a single pass.</li>
  <li><b>Lazy evaluation</b>: adapters build a stack-allocated state machine. Nothing executes until a <em>consuming</em> adapter is called — <code>.sum()</code>, <code>.count()</code>, <code>.for_each()</code>, <code>.collect()</code>.</li>
</ul>
<p><b>The one allocator</b>: <code>.collect::&lt;Vec&lt;_&gt;&gt;()</code> triggers a heap allocation. Every adapter before it is stack-only. This means long iterator chains are O(1) space (excluding output).</p>
<p><b>Avoid</b> <code>.clone()</code> in hot chains — it defeats the zero-copy story. Use <code>.copied()</code> for <code>Copy</code> types instead of <code>.cloned()</code>.</p>"#,
            code: r#"use std::time::Instant;

fn main() {
    let n: u64 = 1_000_000;

    // --- Iterator chain: filter + map + sum ---
    let t0 = Instant::now();
    let iter_sum: u64 = (0..n)
        .filter(|x| x % 3 == 0)
        .map(|x| x * x)
        .sum();
    let iter_us = t0.elapsed().as_micros();

    // --- Equivalent hand-written loop ---
    let t1 = Instant::now();
    let mut loop_sum: u64 = 0;
    for x in 0..n {
        if x % 3 == 0 {
            loop_sum += x * x;
        }
    }
    let loop_us = t1.elapsed().as_micros();

    println!("Iterator sum : {} ({} µs)", iter_sum, iter_us);
    println!("Loop sum     : {} ({} µs)", loop_sum, loop_us);
    println!("Results match: {}", iter_sum == loop_sum);
    println!("(timings vary — both compile to near-identical machine code)");

    // --- collect() is the only adapter that heap-allocates ---
    let squares: Vec<u64> = (0..10u64).map(|x| x * x).collect();
    println!("\ncollect result: {:?}", squares);

    // --- Use .copied() not .cloned() for Copy element types ---
    let data = [1u32, 2, 3, 4, 5];
    let doubled: Vec<u32> = data.iter().copied().map(|x| x * 2).collect();
    println!("doubled     : {:?}", doubled);

    // --- Consuming adapters that do NOT allocate ---
    let max_sq = (1u64..=100).map(|x| x * x).max().unwrap_or(0);
    let any_big = (0u64..50).map(|x| x * x).any(|x| x > 2000);
    println!("max square 1..=100 : {}", max_sq);
    println!("any square > 2000  : {}", any_big);
}"#,
            notes: vec![
                "Iterator adapters are lazy: they build a stack-allocated state machine and run in one pass when consumed.",
                "Monomorphization means zero dynamic dispatch — the closure code is inlined directly at the call site.",
                ".copied() for &Copy items; .cloned() for &Clone items (allocates if T allocates).",
                ".collect::<Vec<_>>() is the only common adapter that heap-allocates — everything else is free.",
                "Interview tip: 'zero-cost abstraction' means the high-level code compiles to the same asm as manual code.",
            ],
        },
        Lesson {
            id: "allocations",
            category: "Memory & Performance",
            title: "Reducing Allocations",
            description: r#"<p>Heap allocation is orders of magnitude slower than stack work — a cache-miss allocation can stall the CPU for hundreds of nanoseconds. In hot paths, allocation pressure matters.</p>
<h3>Key patterns</h3>
<ul>
  <li><b>Pre-allocate</b>: <code>Vec::with_capacity(n)</code> / <code>String::with_capacity(n)</code> — one allocation instead of O(log n) reallocations as the buffer doubles.</li>
  <li><b>Borrow, don't clone</b>: take <code>&amp;str</code> instead of <code>String</code>, <code>&amp;[T]</code> instead of <code>Vec&lt;T&gt;</code>. Auto-deref handles the coercion at call sites.</li>
  <li><b>Return slices</b>: functions that search or parse can return <code>&amp;str</code> / <code>&amp;[T]</code> into the input — zero allocation, zero copy.</li>
  <li><b><code>Cow&lt;str&gt;</code></b>: <code>std::borrow::Cow</code> is either <code>Borrowed(&amp;str)</code> or <code>Owned(String)</code>. Use it when a function <em>usually</em> returns a borrow but <em>sometimes</em> must allocate (percent-decoding, escaping, normalisation).</li>
  <li><b>Iterator pipelines</b>: process data without materialising intermediate <code>Vec</code>s.</li>
</ul>
<p><b>Measure first</b>: use <code>cargo-flamegraph</code> or <code>perf</code> before micro-optimising. The allocator is fast for small, short-lived objects. Over-optimising early adds complexity without payoff.</p>"#,
            code: r#"use std::borrow::Cow;

// Returns a slice INTO the input — zero allocation.
fn largest_word(s: &str) -> &str {
    s.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

// Cow<str>: borrows when no change needed, allocates only when modified.
fn ensure_no_leading_space(s: &str) -> Cow<str> {
    if s.starts_with(' ') {
        Cow::Owned(s.trim_start().to_owned()) // allocates once
    } else {
        Cow::Borrowed(s)                       // zero allocation
    }
}

fn main() {
    // Pre-allocate to avoid O(log n) reallocations
    let mut squares: Vec<u64> = Vec::with_capacity(100);
    for i in 0u64..100 {
        squares.push(i * i);
    }
    println!("len={}, capacity={}", squares.len(), squares.capacity());

    // &str parameter: caller passes a reference, no clone needed
    let text = "the quick brown fox jumps over the lazy dog";
    println!("Largest word: '{}'  (zero allocations)", largest_word(text));

    // Cow — fast path returns a borrow, slow path allocates
    let a = ensure_no_leading_space("  hello"); // allocates
    let b = ensure_no_leading_space("world");   // borrows
    println!("Cow a: '{}', b: '{}'", a, b);

    // &str params: both calls borrow — no clone, no allocation
    fn greet(name: &str) { println!("Hello, {}!", name); }
    let owned = String::from("Alice");
    greet(&owned); // coerced to &str
    greet("Bob");  // literal &str

    // Iterator pipeline: no intermediate Vec at all
    let sum_of_even_squares: u64 = (1u64..=1000)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("Sum of even squares 1..=1000: {}", sum_of_even_squares);

    // String::with_capacity avoids reallocations for known-size output
    let mut s = String::with_capacity(32);
    s.push_str("Hello");
    s.push_str(", world!");
    println!("{} (cap={})", s, s.capacity());
}"#,
            notes: vec![
                "Vec::with_capacity(n) avoids O(log n) reallocations; capacity doubles each time by default.",
                "Prefer &str / &[T] in function signatures. Callers can pass String/Vec — auto-deref handles coercion.",
                "Cow<'a, str> is Borrowed(&'a str) | Owned(String). It only allocates when you need to mutate.",
                "Returning &str from a parse/search function is zero-cost — just a pointer+length into the input buffer.",
                "Interview tip: mention 'use with_capacity when size is known upfront' as a classic allocation answer.",
            ],
        },
    ]
}
