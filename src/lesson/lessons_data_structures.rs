use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "vec",
            category: "Data Structures",
            title: "Vec<T>: Growable Arrays",
            description: r##"
<p><code>Vec&lt;T&gt;</code> is Rust's workhorse sequence type — a heap-allocated, contiguous, growable array.
It is the direct equivalent of JavaScript's <code>Array</code> or TypeScript's <code>T[]</code>, but with
explicit ownership semantics and no hidden resizing surprises.</p>

<h3>Creation</h3>
<ul>
  <li><code>Vec::new()</code> — empty, no allocation yet.</li>
  <li><code>vec![1, 2, 3]</code> — literal macro, allocates immediately.</li>
  <li><code>Vec::with_capacity(n)</code> — preallocates <em>n</em> slots; avoids repeated reallocations
    when you know the approximate final size.</li>
  <li><code>iter.collect::&lt;Vec&lt;T&gt;&gt;()</code> — from any iterator.</li>
</ul>

<h3>Indexing</h3>
<p><code>v[i]</code> — panics on out-of-bounds (like a TypeScript array without bounds checking).
<code>v.get(i)</code> — returns <code>Option&lt;&amp;T&gt;</code>; never panics. Prefer <code>get</code> in library code.</p>

<h3>Capacity vs Length</h3>
<p><code>len</code> is the number of elements; <code>capacity</code> is the number of slots allocated before a
reallocation occurs. Doubling strategy is used on growth. Pre-allocate with
<code>with_capacity</code> when the size is known — it is the same insight as pre-sizing a
TypeScript <code>new Array(n)</code> but with guaranteed contiguous memory.</p>

<h3>Iterator adapters</h3>
<p>Rust's iterator model is lazy and zero-cost: <code>.iter()</code>, <code>.filter()</code>, <code>.map()</code>,
<code>.sum()</code>, <code>.collect()</code>. These are the Rust equivalent of TypeScript's Array methods,
but they compose without intermediate array allocations.</p>
"##,
            code: r##"fn main() {
    // ── Creation ─────────────────────────────────────────────────────────
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("pushed: {v:?}");

    let fruits = vec!["apple", "banana", "cherry"];
    println!("fruits: {fruits:?}");

    // ── Indexing ─────────────────────────────────────────────────────────
    println!("first: {}", fruits[0]);          // panics if out of range

    match fruits.get(10) {
        Some(f) => println!("got: {f}"),
        None    => println!("index 10 is out of range"),
    }

    // ── pop removes and returns the last element ─────────────────────────
    let mut stack = vec![1, 2, 3];
    while let Some(n) = stack.pop() {
        print!("{n} ");
    }
    println!();

    // ── Iteration ────────────────────────────────────────────────────────
    let data: Vec<i32> = (1..=10).collect();

    let sum: i32 = data.iter().sum();
    let evens: Vec<i32> = data.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let doubled: Vec<i32> = data.iter().map(|&x| x * 2).collect();

    println!("sum={sum}, evens={evens:?}");
    println!("doubled={doubled:?}");

    // ── with_capacity: avoid reallocation ───────────────────────────────
    let mut prealloc: Vec<i32> = Vec::with_capacity(8);
    println!("empty — len={}, cap={}", prealloc.len(), prealloc.capacity());
    for i in 0..8 {
        prealloc.push(i);
    }
    println!("filled — len={}, cap={}", prealloc.len(), prealloc.capacity());

    // ── Sorting and deduplication ────────────────────────────────────────
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    nums.sort();
    nums.dedup();
    println!("sorted+dedup: {nums:?}");

    // ── Retain: in-place filter ──────────────────────────────────────────
    let mut vals = vec![1, 2, 3, 4, 5, 6];
    vals.retain(|&x| x % 2 != 0);
    println!("odds only: {vals:?}");
}
"##,
            notes: vec![
                "Vec<T> is a heap-allocated, growable, contiguous array — Rust's equivalent of T[].",
                "v[i] panics on out-of-bounds; v.get(i) returns Option<&T> — prefer get() in library code.",
                "with_capacity(n) preallocates n slots, avoiding repeated heap reallocations.",
                "Iterator adapters (filter, map, sum, collect) are lazy and zero-cost — no intermediate allocations.",
                "push/pop give O(1) amortized stack semantics; insert/remove at arbitrary index are O(n).",
            ],
        },
        Lesson {
            id: "arrays-slices",
            category: "Data Structures",
            title: "Arrays and Slices",
            description: r##"
<p>Rust has two array-like types that live entirely on the stack: <code>[T; N]</code> (fixed-size array)
and <code>&amp;[T]</code> (borrowed slice). These are distinct from <code>Vec&lt;T&gt;</code>, which lives on the heap.</p>

<h3>[T; N] — Fixed-size array</h3>
<p>The size <code>N</code> is part of the type — <code>[i32; 4]</code> and <code>[i32; 5]</code> are <em>different types</em>.
Arrays are stack-allocated: ideal for small, fixed-size data (lookup tables, buffers, SIMD).
In TypeScript there is no equivalent; all arrays are heap objects.</p>
<ul>
  <li><code>let a: [i32; 4] = [1, 2, 3, 4];</code></li>
  <li><code>let zeros = [0u8; 64];</code> — 64 zeroed bytes</li>
</ul>

<h3>&amp;[T] — Slice (borrowed view)</h3>
<p>A fat pointer (ptr + len) into a contiguous sequence owned by someone else — an array, a
<code>Vec</code>, or part of another slice. Functions should accept <code>&amp;[T]</code> rather than <code>&amp;Vec&lt;T&gt;</code>
to work with both sources without any allocation.</p>

<h3>When to choose array over Vec</h3>
<ul>
  <li>Size is known at compile time and small.</li>
  <li>Stack allocation required (embedded, kernel code, WASM tight loops).</li>
  <li>Guaranteed no heap allocation (no allocator dependency).</li>
</ul>
<p>For anything dynamically sized or whose size is determined at runtime, use <code>Vec&lt;T&gt;</code>.</p>
"##,
            code: r##"fn print_all(label: &str, s: &[i32]) {
    // Accepts both [i32; N] arrays and Vec<i32> — same function
    print!("{label}: ");
    for x in s {
        print!("{x} ");
    }
    println!();
}

fn sum(s: &[i32]) -> i32 {
    s.iter().sum()
}

fn main() {
    // ── Fixed-size array ─────────────────────────────────────────────────
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {a:?}");
    println!("length: {}", a.len());

    // Uniform initialization
    let zeros = [0_u8; 8];
    println!("zeros: {zeros:?}");

    // ── Slicing ──────────────────────────────────────────────────────────
    let mid: &[i32] = &a[1..4]; // [2, 3, 4] — no allocation
    println!("middle slice: {mid:?}");
    print_all("all",  &a);       // array coerces to &[i32]
    print_all("tail", &a[2..]);

    // ── Vec also coerces to &[T] ─────────────────────────────────────────
    let v: Vec<i32> = vec![10, 20, 30, 40, 50];
    print_all("vec", &v);
    print_all("vec mid", &v[1..4]);
    println!("sum of vec: {}", sum(&v));

    // ── Array → Vec ───────────────────────────────────────────────────────
    let v2: Vec<i32> = a.to_vec();
    println!("array as vec: {v2:?}");

    // ── 2-D array ────────────────────────────────────────────────────────
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    for row in &matrix {
        println!("{row:?}");
    }

    // ── Pattern: pass &[T] not &Vec<T> in function signatures ───────────
    // Both calls below work with the same sum() function:
    println!("sum array: {}", sum(&a));
    println!("sum vec:   {}", sum(&v));
}
"##,
            notes: vec![
                "[T; N] is a fixed-size, stack-allocated array — the size N is part of the type.",
                "&[T] is a borrowed slice (fat pointer) that works for both arrays and Vecs.",
                "Function parameters should use &[T] rather than &Vec<T> to accept both sources.",
                "Use arrays for compile-time-known small sizes; use Vec for dynamic or runtime-sized data.",
                "Slices are zero-copy views — &a[1..4] is a pointer + length, not a new allocation.",
            ],
        },
        Lesson {
            id: "hashmap",
            category: "Data Structures",
            title: "HashMap and BTreeMap",
            description: r##"
<p>Rust's standard library provides two map types covering the two most common needs: fast O(1) access
and sorted iteration.</p>

<h3>HashMap&lt;K, V&gt;</h3>
<p>Hash map with <b>randomized seed</b> by default (SipHash) to prevent hash-flooding DoS attacks.
Iteration order is <b>not deterministic</b> — this is different from JavaScript objects, which preserve
insertion order since ES2015. Always use <code>BTreeMap</code> or explicit sorting when order matters.</p>
<ul>
  <li><code>insert(k, v)</code> — inserts; returns <code>Option&lt;V&gt;</code> of any previous value.</li>
  <li><code>get(&amp;k)</code> — returns <code>Option&lt;&amp;V&gt;</code>; key is borrowed, not consumed.</li>
  <li><code>contains_key(&amp;k)</code> — boolean check.</li>
  <li><code>remove(&amp;k)</code> — returns <code>Option&lt;V&gt;</code>.</li>
</ul>

<h3>The entry API — upsert pattern</h3>
<p><code>map.entry(key).or_insert(default)</code> is the idiomatic upsert: insert the default only if
the key is absent, then return a mutable reference to the value. This avoids a double-lookup.
<code>or_insert_with(|| expensive())</code> lazily computes the default.</p>
<pre>
*word_count.entry(word).or_insert(0) += 1;
</pre>

<h3>BTreeMap&lt;K, V&gt;</h3>
<p>B-tree backed map with <b>sorted key order</b>. Iteration is always in ascending key order.
O(log n) operations vs HashMap's O(1) — choose BTreeMap when you need range queries or
reproducible iteration.</p>
"##,
            code: r##"use std::collections::{BTreeMap, HashMap};

fn main() {
    // ── Basic HashMap operations ──────────────────────────────────────────
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"),   85);
    scores.insert(String::from("Carol"), 92);

    // get returns Option<&V> — the key is borrowed, not moved
    if let Some(s) = scores.get("Alice") {
        println!("Alice: {s}");
    }

    // Iteration order is NOT guaranteed
    for (name, score) in &scores {
        println!("  {name}: {score}");
    }

    // ── Entry API: safe upsert ────────────────────────────────────────────
    // Insert 0 only if the key is absent, then add a bonus
    *scores.entry(String::from("Alice")).or_insert(0) += 10;
    scores.entry(String::from("Dave")).or_insert(0);  // new entry, value=0

    println!("Alice after bonus: {}", scores["Alice"]);
    println!("Dave (new): {}", scores["Dave"]);

    // ── Classic word count using entry API ───────────────────────────────
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    println!("\nword counts (unordered):");
    for (w, c) in &word_count {
        if *c > 1 {
            println!("  '{w}' appears {c}x");
        }
    }

    // ── BTreeMap: sorted key iteration ───────────────────────────────────
    let mut sorted: BTreeMap<&str, u32> = word_count
        .iter()
        .map(|(&k, &v)| (k, v))
        .collect();

    println!("\nword counts (sorted alphabetically):");
    for (w, c) in &sorted {
        println!("  {w}: {c}");
    }

    // BTreeMap supports range queries (half-open ranges work directly)
    println!("\nwords from 't' up to 'z':");
    for (w, c) in sorted.range("t".."z") {
        println!("  {w}: {c}");
    }
}
"##,
            notes: vec![
                "HashMap uses a randomized seed (SipHash) — iteration order is non-deterministic.",
                "get(&k) returns Option<&V>; use [] indexing only when you're sure the key exists (panics otherwise).",
                "The entry API (*map.entry(k).or_insert(default) += 1) is the idiomatic single-lookup upsert.",
                "BTreeMap stores keys in sorted order — use for range queries or when iteration order matters.",
                "Prefer &str or integer keys when possible; String keys cause a heap allocation per lookup with get().",
            ],
        },
        Lesson {
            id: "structs",
            category: "Data Structures",
            title: "Structs",
            description: r##"
<p>Structs are Rust's primary way to group related data into a named type. They are analogous to
TypeScript interfaces or classes (without the class hierarchy), but with explicit ownership rules
for each field.</p>

<h3>Three flavors</h3>
<ul>
  <li><b>Named-field struct</b> — the common case: <code>struct User { id: u64, name: String }</code></li>
  <li><b>Tuple struct</b> — fields accessed by position: <code>struct Point(f64, f64);</code>
    Useful for newtype wrappers that add type safety: <code>struct Meters(f64);</code></li>
  <li><b>Unit struct</b> — no fields: <code>struct Marker;</code> Used as zero-sized marker types for
    generics or trait implementations.</li>
</ul>

<h3>Construction conveniences</h3>
<p><b>Field-init shorthand</b>: when a local variable has the same name as a field, you can omit
the <code>: value</code> part — <code>User { id, name }</code> instead of <code>User { id: id, name: name }</code>.
This mirrors TypeScript's shorthand object properties.</p>
<p><b>Struct update syntax</b>: <code>User { id: 2, ..other }</code> copies (or moves) remaining fields
from <code>other</code>. If any copied field is non-<code>Copy</code>, <code>other</code> is partially moved.</p>

<h3>Methods</h3>
<p>Methods go in an <code>impl</code> block. <code>&amp;self</code> for read, <code>&amp;mut self</code> for mutation,
<code>self</code> to consume. Convention: <code>fn new(...) -&gt; Self</code> as an associated function (no receiver),
called as <code>Type::new(...)</code>. There is no class keyword; <code>impl</code> blocks can be added from
anywhere (in the same crate), and a type can have multiple <code>impl</code> blocks.</p>
"##,
            code: r##"// ── Named-field struct ───────────────────────────────────────────────────
struct User {
    id:     u64,
    name:   String,
    email:  String,
    active: bool,
}

impl User {
    fn new(id: u64, name: &str, email: &str) -> Self {
        // Field-init shorthand: `id` instead of `id: id`
        let name  = name.to_string();
        let email = email.to_string();
        User { id, name, email, active: true }
    }

    fn display(&self) {
        println!(
            "#{} {} <{}> active={}",
            self.id, self.name, self.email, self.active
        );
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

// ── Tuple struct (newtype pattern) ───────────────────────────────────────────
struct Point(f64, f64);

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

// ── Unit struct (marker type) ────────────────────────────────────────────────
struct AdminOnly;

fn main() {
    let alice = User::new(1, "Alice", "alice@example.com");
    alice.display();

    // ── Struct update syntax ─────────────────────────────────────────────
    // Fields with non-Copy types (String) are MOVED from alice into bob.
    // Call alice.display() BEFORE creating bob, or alice.name is moved away.
    let bob = User {
        id:    2,
        name:  String::from("Bob"),
        email: String::from("bob@example.com"),
        ..alice // 'active' (bool = Copy) comes from alice
    };
    bob.display();
    // alice.name is now moved; alice.id and alice.active are still accessible
    println!("alice.id still accessible: {}", alice.id);

    // ── Mutable method ───────────────────────────────────────────────────
    let mut carol = User::new(3, "Carol", "carol@example.com");
    carol.display();
    carol.deactivate();
    carol.display();

    // ── Tuple struct ─────────────────────────────────────────────────────
    let origin = Point(0.0, 0.0);
    let tip    = Point(3.0, 4.0);
    println!("distance: {:.1}", origin.distance(&tip));

    // Access fields by index
    println!("tip x={}, y={}", tip.0, tip.1);

    // ── Unit struct ───────────────────────────────────────────────────────
    let _token = AdminOnly;
    println!("AdminOnly token created (zero-sized type)");
}
"##,
            notes: vec![
                "Three struct flavors: named-field (most common), tuple struct (newtype pattern), unit struct (markers).",
                "Methods live in impl blocks; &self reads, &mut self mutates, self consumes — mirrors Rust's ownership.",
                "Associated functions (no self) serve as constructors: Type::new(...) is the standard convention.",
                "Field-init shorthand (User { id, name }) and update syntax (..other) reduce boilerplate.",
                "Struct update syntax partially moves non-Copy fields from the source — the source binding may become invalid.",
            ],
        },
        Lesson {
            id: "enums",
            category: "Data Structures",
            title: "Enums and Algebraic Data Types",
            description: r##"
<p>Rust enums are <b>algebraic data types (ADTs)</b> — each variant can carry different data.
This is the feature TypeScript engineers most envy: discriminated unions baked into the type system,
with exhaustive pattern matching enforced by the compiler.</p>

<h3>Enum variants can hold data</h3>
<p>Unlike TypeScript's <code>enum</code> (which is just integers or strings), Rust enum variants can each
carry different shapes of data:</p>
<ul>
  <li><code>Variant</code> — unit (no data), like a flag</li>
  <li><code>Variant(T)</code> — single unnamed field</li>
  <li><code>Variant(T1, T2)</code> — tuple of fields</li>
  <li><code>Variant { field: T }</code> — named fields (inline struct)</li>
</ul>
<p>TypeScript achieves the same with discriminated unions (<code>type Event = ClickEvent | KeyEvent</code>),
but Rust's <code>match</code> enforces exhaustiveness at compile time — no <code>never</code> hacks needed.</p>

<h3>match must be exhaustive</h3>
<p>Every possible variant must be handled. Add new variants to an enum and every match
site becomes a compile error until you handle it — a safety net TypeScript switch-case cannot provide.</p>

<h3>Option and Result are enums</h3>
<p><code>Option&lt;T&gt;</code> is <code>enum Option { Some(T), None }</code> — Rust's null-safe value.
<code>Result&lt;T, E&gt;</code> is <code>enum Result { Ok(T), Err(E) }</code> — the error-handling primitive.
Both are standard-library enums, not special syntax.</p>
"##,
            code: r##"#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },  // named fields (inline struct)
    KeyDown(char),              // single field
    Resize(u32, u32),           // tuple of fields
    Close,                      // unit variant (no data)
}

fn handle(event: &Event) {
    // match is exhaustive — all variants must be covered
    match event {
        Event::Click { x, y }  => println!("click at ({x},{y})"),
        Event::KeyDown(ch)      => println!("key pressed: '{ch}'"),
        Event::Resize(w, h)     => println!("resized to {w}x{h}"),
        Event::Close            => println!("window closed"),
    }
}

// Result<T, E> is itself an enum: Ok(T) | Err(E)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

// Recursive enum — each node is either a value or a branch
#[derive(Debug)]
enum Tree {
    Leaf(i32),
    Branch(Box<Tree>, Box<Tree>),
}

impl Tree {
    fn sum(&self) -> i32 {
        match self {
            Tree::Leaf(n)           => *n,
            Tree::Branch(l, r)      => l.sum() + r.sum(),
        }
    }
}

fn main() {
    let events = vec![
        Event::Click { x: 100, y: 200 },
        Event::KeyDown('r'),
        Event::Resize(1920, 1080),
        Event::Close,
    ];
    for e in &events {
        handle(e);
    }

    // ── Option<T>: Some(T) | None ────────────────────────────────────────
    let maybe: Option<i32> = Some(42);
    if let Some(n) = maybe {
        println!("got {n}");
    }

    let nothing: Option<i32> = None;
    println!("unwrap_or: {}", nothing.unwrap_or(-1));
    println!("map:       {:?}", maybe.map(|n| n * 2)); // Some(84)

    // ── Result<T, E>: Ok(T) | Err(E) ─────────────────────────────────────
    match divide(10.0, 3.0) {
        Ok(v)  => println!("10/3 = {v:.4}"),
        Err(e) => println!("error: {e}"),
    }
    match divide(5.0, 0.0) {
        Ok(v)  => println!("{v}"),
        Err(e) => println!("error: {e}"),
    }

    // ── Recursive enum ────────────────────────────────────────────────────
    let tree = Tree::Branch(
        Box::new(Tree::Branch(
            Box::new(Tree::Leaf(1)),
            Box::new(Tree::Leaf(2)),
        )),
        Box::new(Tree::Leaf(3)),
    );
    println!("tree sum: {}", tree.sum()); // 6
}
"##,
            notes: vec![
                "Rust enums are algebraic data types — each variant can carry different shapes of data.",
                "match is exhaustive: add a new variant and every unhandled match site becomes a compile error.",
                "Option<T> = Some(T) | None; Result<T, E> = Ok(T) | Err(E) — both are standard-library enums.",
                "Use if let Some(x) = opt for single-variant matching without writing a full match block.",
                "Recursive enums require Box<Self> to give the compiler a finite, known size.",
            ],
        },
        Lesson {
            id: "tuples-and-destructuring",
            category: "Data Structures",
            title: "Tuples and Destructuring",
            description: r##"
<p>Tuples are fixed-size, heterogeneous sequences — each position can have a different type.
Destructuring is a universal Rust pattern for extracting values from tuples, structs, enums,
and slices in <code>let</code> bindings, function parameters, and <code>match</code> arms.</p>

<h3>Tuples</h3>
<p><code>(i32, &amp;str, f64)</code> — the type is the combination of all element types.
Fields accessed via <code>.0</code>, <code>.1</code>, etc. The empty tuple <code>()</code> is the
<em>unit type</em> — Rust's equivalent of <code>void</code>, used as the return type of functions
that return nothing meaningful.</p>

<h3>Returning multiple values</h3>
<p>Tuples are the idiomatic way to return multiple values from a function — no need for out-parameters
or wrapper objects. TypeScript users often do this with object literals; Rust uses tuples for small,
ad-hoc groupings when a named struct would be overkill.</p>

<h3>Destructuring</h3>
<p>Destructuring works in <code>let</code>, <code>match</code>, function arguments, and <code>for</code> loops.
Patterns can be nested arbitrarily. The <code>..</code> syntax ignores remaining fields in a struct
or remaining elements in a tuple. <code>_</code> ignores a single field without binding it.</p>

<h3>Struct destructuring</h3>
<p><code>let User { name, id, .. } = user;</code> extracts named fields. This is the equivalent of
TypeScript's object destructuring (<code>const { name, id } = user</code>), and it works in the same
contexts, including <code>match</code> arms and function parameters.</p>
"##,
            code: r##"fn min_max(data: &[i32]) -> (i32, i32) {
    // Return two values without a wrapper struct
    let mut mn = data[0];
    let mut mx = data[0];
    for &n in &data[1..] {
        if n < mn { mn = n; }
        if n > mx { mx = n; }
    }
    (mn, mx)
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    // ── Tuple creation and positional access ─────────────────────────────
    let t: (i32, &str, f64) = (42, "hello", 3.14);
    println!("{} {} {:.2}", t.0, t.1, t.2);

    // ── let destructuring ────────────────────────────────────────────────
    let (a, b, c) = t;
    println!("a={a}, b={b}, c={c:.2}");

    // ── Multiple return values ───────────────────────────────────────────
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let (min, max) = min_max(&data);
    println!("min={min}, max={max}");

    // ── match destructuring — pattern on tuples ─────────────────────────
    let status = (200_u16, "OK");
    match status {
        (200, msg) => println!("success: {msg}"),
        (404, msg) => println!("not found: {msg}"),
        (code, msg) => println!("other {code}: {msg}"),
    }

    // ── Nested tuple destructuring ───────────────────────────────────────
    let segment = ((0, 0), (3, 4));
    let ((x1, y1), (x2, y2)) = segment;
    println!("from ({x1},{y1}) to ({x2},{y2})");

    // ── Struct destructuring ─────────────────────────────────────────────
    let color = Color { r: 255, g: 128, b: 0 };
    let Color { r, g, b } = color;
    println!("rgb({r},{g},{b})");

    let color2 = Color { r: 0, g: 200, b: 50 };
    let Color { r: red, .. } = color2; // .. ignores g and b
    println!("red channel: {red}");

    // ── Destructuring in for loops ───────────────────────────────────────
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    for (n, ch) in &pairs {
        println!("{n} -> {ch}");
    }

    // ── _ to ignore a field ──────────────────────────────────────────────
    let (first, _, last) = (10, 20, 30);
    println!("first={first}, last={last}");

    // ── Unit type () as return value ─────────────────────────────────────
    fn do_nothing() -> () {}
    let unit = do_nothing();
    println!("unit == (): {}", unit == ());
}
"##,
            notes: vec![
                "Tuples are fixed-size, heterogeneous sequences; fields accessed by .0, .1, etc.",
                "Returning a tuple is the idiomatic way to return multiple values from a function.",
                "Destructuring works in let, match arms, for loops, and function parameters.",
                ".. ignores remaining struct fields; _ ignores a single position without binding it.",
                "The unit type () is Rust's void — the implicit return type of functions that return nothing.",
            ],
        },
    ]
}
