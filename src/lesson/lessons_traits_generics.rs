use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "methods-impl",
            category: "Traits & Generics",
            title: "Methods with impl Blocks",
            description: r#"<p>
  <code>impl</code> blocks attach methods and associated functions to a type.
  In TypeScript you'd write a <code>class</code> with methods; in Rust you keep the
  data (a <code>struct</code> or <code>enum</code>) and the behaviour (<code>impl</code>) separate.
</p>

<h3>Receiver types</h3>
<ul>
  <li><code>&amp;self</code> — immutable borrow; the most common. The caller retains ownership.</li>
  <li><code>&amp;mut self</code> — mutable borrow; lets the method modify the value.</li>
  <li><code>self</code> — consumes (moves) the value into the method. The caller can no longer use it.</li>
  <li><b>No receiver</b> — "associated function" (like a static method). Called as <code>Type::fn_name()</code>.</li>
</ul>

<h3>Multiple impl blocks</h3>
<p>
  You can split an <code>impl</code> across multiple blocks — Rust merges them at compile time.
  This is useful for grouping constructors, business logic, and trait implementations separately.
</p>

<h3>TypeScript parallel</h3>
<p>
  <code>fn new(...) -> Self</code> is the idiomatic Rust constructor, equivalent to a TS
  <code>static create()</code> factory method. Rust has no <code>new</code> keyword —
  <code>new</code> is just a naming convention, not syntax.
</p>"#,
            code: r#"struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function — no `self`, called as Rectangle::new(...)
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    // &self: immutable borrow — caller keeps ownership
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // &mut self: mutable borrow — can modify fields
    fn set_width(&mut self, w: f64) {
        self.width = w;
    }

    // self: moves the rectangle — caller CANNOT use `r` after this call
    fn into_unit_square(self) -> Rectangle {
        Rectangle { width: 1.0, height: 1.0 }
    }
}

// Multiple impl blocks are allowed — Rust merges them
impl Rectangle {
    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

fn main() {
    let mut r = Rectangle::new(4.0, 3.0);
    println!("area:      {}", r.area());      // 12
    println!("perimeter: {}", r.perimeter()); // 14

    r.set_width(5.0);
    println!("new area:  {}", r.area());      // 15

    let unit = r.into_unit_square(); // r is moved — r is gone after this line
    println!("is square: {}", unit.is_square()); // true
}"#,
            notes: vec![
                "&self borrows immutably, &mut self borrows mutably, self consumes. Pick the least powerful receiver that works.",
                "Associated functions (no self) are called with Type::fn_name() — fn new() is the conventional constructor.",
                "Multiple impl blocks are valid and merge at compile time — a common pattern when also implementing traits.",
                "Rust has no constructors or destructors in the C++ sense; implement the Drop trait for cleanup logic.",
            ],
        },
        Lesson {
            id: "traits",
            category: "Traits & Generics",
            title: "Traits",
            description: r#"<p>
  A <b>trait</b> is a contract of shared behaviour — a set of method signatures a type must implement.
  Think of it as a TypeScript <code>interface</code>, but with one key difference:
  <b>you must write <code>impl Trait for Type</code> explicitly</b>.
</p>

<h3>Default methods</h3>
<p>
  Traits can provide default implementations that implementors inherit for free.
  Implementors can override them if needed. This is similar to abstract classes in OOP,
  but without inheritance hierarchies.
</p>

<h3>Dispatch</h3>
<p>
  By default, trait method calls are resolved at <b>compile time</b> (static dispatch).
  The compiler knows the exact concrete type and calls the method directly — zero overhead.
  Dynamic dispatch via <code>dyn Trait</code> is covered in a later lesson.
</p>

<h3>TS / Go comparison</h3>
<ul>
  <li><b>TypeScript interface:</b> structural typing, implicit satisfaction, describes object shapes.</li>
  <li><b>Go interface:</b> structural + implicit — any type with the right methods satisfies the interface automatically.</li>
  <li><b>Rust trait:</b> nominal + explicit — you must write <code>impl Trait for Type</code>.
      Accidental interface satisfaction is impossible, which makes code more intentional and readable.</li>
</ul>"#,
            code: r#"use std::f64::consts::PI;

trait Shape {
    // Required methods — implementors MUST provide these
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Default method — implementors MAY override, or inherit for free
    fn name(&self) -> &str { "shape" }

    // Default method that composes other methods
    fn describe(&self) {
        println!("{}: area={:.2}, perimeter={:.2}",
            self.name(), self.area(), self.perimeter());
    }
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64      { PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * PI * self.radius }
    fn name(&self) -> &str     { "circle" }
}

impl Shape for Square {
    fn area(&self) -> f64      { self.side * self.side }
    fn perimeter(&self) -> f64 { 4.0 * self.side }
    fn name(&self) -> &str     { "square" }
    // describe() is inherited — no override needed
}

// `impl Trait` in argument position: static dispatch, sugar for a generic
fn print_area(s: &impl Shape) {
    println!("{} area = {:.2}", s.name(), s.area());
}

fn main() {
    let c  = Circle { radius: 3.0 };
    let sq = Square { side: 4.0 };

    c.describe();   // circle: area=28.27, perimeter=18.85
    sq.describe();  // square: area=16.00, perimeter=16.00

    print_area(&c);
    print_area(&sq);
}"#,
            notes: vec![
                "Traits must be in scope (use ...) to call their methods, even if the type implements them.",
                "Default methods let you add behaviour to a trait without breaking existing implementors.",
                "`impl Trait` in argument position is sugar for a generic: fn f(x: &impl T) is fn f<X: T>(x: &X).",
                "Unlike Go, accidental interface satisfaction is impossible in Rust — you must write `impl` explicitly.",
                "Rust traits can define associated types and associated constants, not just methods.",
            ],
        },
        Lesson {
            id: "trait-bounds",
            category: "Traits & Generics",
            title: "Generic Functions & Trait Bounds",
            description: r#"<p>
  Generics let you write one function that works for many types.
  In TypeScript you'd write <code>function f&lt;T&gt;(x: T)</code>; Rust uses the same syntax
  but requires you to declare <b>what the type can do</b> via <b>trait bounds</b>.
  Without a bound, generic code can do almost nothing with <code>T</code>.
</p>

<h3>Syntax forms</h3>
<ul>
  <li><code>fn f&lt;T: TraitA&gt;(x: T)</code> — inline bound</li>
  <li><code>fn f&lt;T: TraitA + TraitB&gt;(x: T)</code> — multiple bounds with <code>+</code></li>
  <li><code>where T: TraitA + TraitB</code> — <code>where</code> clause; preferred when bounds get long</li>
</ul>

<h3>Common standard bounds</h3>
<ul>
  <li><code>PartialOrd</code> — supports <code>&lt;</code> and <code>&gt;</code> (floats, strings, numbers)</li>
  <li><code>Ord</code> — total ordering; required for <code>.sort()</code></li>
  <li><code>Display</code> — supports <code>{}</code> formatting</li>
  <li><code>Clone</code> — supports <code>.clone()</code></li>
</ul>

<h3>TypeScript parallel</h3>
<p>
  TypeScript's <code>T extends Comparable</code> is roughly equivalent.
  Rust is stricter: you must list every capability you need; the compiler won't infer it from usage.
</p>"#,
            code: r#"use std::fmt::Display;

// T must support > comparison (PartialOrd)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Multiple bounds: T must be both comparable and printable
fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    println!("The largest is: {}", largest(list));
}

// where clause — same semantics, easier to read for complex bounds
fn compare_and_display<T, U>(t: &T, u: &U)
where
    T: Display + PartialOrd,
    U: Display,
{
    println!("Comparing {} with {}", t, u);
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers)); // 100

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars)); // y

    print_largest(&numbers);
    print_largest(&chars);

    compare_and_display(&42_i32, &"hello");
}"#,
            notes: vec![
                "T: PartialOrd means T supports < and > but may not have a total order (floats have NaN). Use T: Ord for total order.",
                "where clauses are preferred when bounds exceed ~2 traits — they keep the function signature readable.",
                "Generic functions are monomorphised: the compiler generates one specialised copy per concrete type used.",
                "The `+` syntax for multiple bounds works in both inline and where positions: T: Ord + Display + Clone.",
            ],
        },
        Lesson {
            id: "dyn-trait",
            category: "Traits & Generics",
            title: "Trait Objects: Box<dyn Trait>",
            description: r#"<p>
  Rust offers two dispatch strategies for traits: <b>static</b> and <b>dynamic</b>.
  Understanding when to use each is essential for senior-level Rust.
</p>

<h3>Static dispatch (generics / impl Trait)</h3>
<p>
  The compiler knows the exact type at compile time and generates a dedicated copy of the function
  (<em>monomorphisation</em>). Zero runtime overhead. Downside: you can't mix types in one collection.
</p>

<h3>Dynamic dispatch (dyn Trait)</h3>
<p>
  <code>&amp;dyn Trait</code> and <code>Box&lt;dyn Trait&gt;</code> store a <b>fat pointer</b>:
  a pointer to the data plus a pointer to a <b>vtable</b> (a table of function pointers, one per method).
  One version of the code, at the cost of ~2 pointer indirections per call.
</p>

<h3>When to use dyn Trait</h3>
<ul>
  <li>Heterogeneous collections: <code>Vec&lt;Box&lt;dyn Shape&gt;&gt;</code></li>
  <li>Return types where the concrete type is chosen at runtime</li>
  <li>Plugin/handler registries where you don't know the type at compile time</li>
</ul>

<h3>Object safety</h3>
<p>
  Only <b>object-safe</b> traits can be used as <code>dyn Trait</code>.
  A trait is object-safe if its methods don't need to know the concrete type's size —
  e.g., <code>fn clone(&amp;self) -> Self</code> is <b>not</b> object-safe, which is why
  <code>Clone</code> cannot be used as <code>dyn Clone</code>.
</p>

<h3>TypeScript parallel</h3>
<p>
  <code>Box&lt;dyn Trait&gt;</code> is closest to a TypeScript interface reference —
  both dispatch through a vtable (or JS prototype chain) at runtime.
</p>"#,
            code: r#"use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle   { radius: f64 }
struct Square   { side: f64 }
struct Triangle { base: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
    fn name(&self) -> &str { "circle" }
}
impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
    fn name(&self) -> &str { "square" }
}
impl Shape for Triangle {
    fn area(&self) -> f64 { 0.5 * self.base * self.height }
    fn name(&self) -> &str { "triangle" }
}

// Static dispatch — compiler creates one copy per concrete type (fast, inflexible)
fn print_area_static(s: &impl Shape) {
    println!("[static]  {} area = {:.2}", s.name(), s.area());
}

// Dynamic dispatch — one copy, vtable lookup at runtime (flexible)
fn print_area_dynamic(s: &dyn Shape) {
    println!("[dynamic] {} area = {:.2}", s.name(), s.area());
}

fn main() {
    // Heterogeneous collection — only possible with dyn Trait
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle   { radius: 2.0 }),
        Box::new(Square   { side: 3.0 }),
        Box::new(Triangle { base: 4.0, height: 5.0 }),
    ];

    for shape in &shapes {
        print_area_dynamic(shape.as_ref());
    }

    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total area: {:.2}", total);

    // Static dispatch still works for single known types
    print_area_static(&Circle { radius: 1.0 });
}"#,
            notes: vec![
                "Box<dyn Trait> owns the heap-allocated value; &dyn Trait borrows it. Use Box when the collection owns elements.",
                "Static dispatch (generics) is zero-cost; dynamic dispatch (dyn) adds one vtable indirection per call.",
                "A trait is object-safe when all methods can be called without knowing the concrete type's exact size.",
                "Clone, Sized, and traits with generic methods are not object-safe — the compiler will tell you.",
                "Prefer generics for performance-critical paths; prefer dyn for heterogeneous collections and plugin systems.",
            ],
        },
        Lesson {
            id: "common-traits",
            category: "Traits & Generics",
            title: "Derivable Traits (Debug, Clone, Eq, ...)",
            description: r#"<p>
  Rust can automatically implement many standard traits via the <code>#[derive(...)]</code> attribute.
  You only need to write a manual implementation when the default behaviour is wrong or insufficient.
</p>

<h3>The most useful derivable traits</h3>
<ul>
  <li><code>Debug</code> — enables <code>{:?}</code> and <code>{:#?}</code> formatting.
      <b>Always derive this</b> — it's free and invaluable for debugging.</li>
  <li><code>Clone</code> — adds an explicit <code>.clone()</code> method that deep-copies the value.
      A <code>String</code> is <code>Clone</code> but not <code>Copy</code>.</li>
  <li><code>Copy</code> — makes the type implicitly bitwise-copied instead of moved.
      Only valid if all fields are <code>Copy</code>. Cannot coexist with <code>Drop</code>.</li>
  <li><code>PartialEq</code> — enables <code>==</code> and <code>!=</code>.
      "Partial" because <code>NaN != NaN</code> for floats.</li>
  <li><code>Eq</code> — marker that <code>PartialEq</code> is a total equivalence relation (no NaN-like gaps). Requires <code>PartialEq</code>.</li>
  <li><code>Hash</code> — lets the type be used as a <code>HashMap</code> key. Requires <code>Eq</code>.</li>
  <li><code>Default</code> — provides a zero/empty value via <code>Default::default()</code>
      or the <code>..Default::default()</code> struct update syntax.</li>
  <li><code>PartialOrd</code> / <code>Ord</code> — enables <code>&lt;</code>, <code>&gt;</code>, <code>.sort()</code>.
      <code>Ord</code> is total order; requires <code>Eq</code>.</li>
</ul>

<h3>TypeScript parallel</h3>
<p>
  TypeScript has no derive mechanism — you implement <code>equals()</code>, <code>toString()</code>,
  and hashing manually (or lean on a library). Rust's derive macros are a compile-time
  code-generation step that scales to large codebases with zero runtime cost.
</p>"#,
            code: r#"use std::collections::HashMap;

// Derive multiple traits at once — the compiler generates all the impls
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

// Copy types: implicitly bitwise-copied, no move semantics
#[derive(Debug, Clone, Copy, PartialEq)]
struct Color(u8, u8, u8);

fn main() {
    let p1 = Point { x: 3, y: 4 };

    // Clone: explicit deep copy
    let p2 = p1.clone();
    println!("p1 = {:?}", p1);          // Debug: Point { x: 3, y: 4 }
    println!("equal: {}", p1 == p2);    // PartialEq: true

    // Default: zero-value for each field
    let origin: Point = Default::default();
    println!("origin = {:?}", origin);  // Point { x: 0, y: 0 }

    // Struct update syntax: fill unspecified fields from Default
    let p3 = Point { x: 10, ..Default::default() };
    println!("p3 = {:?}", p3);          // Point { x: 10, y: 0 }

    // Copy: implicit bitwise copy — `red` is still valid after assignment
    let red = Color(255, 0, 0);
    let also_red = red;                 // copy, NOT move
    println!("{:?} still valid after copy: {:?}", red, also_red);

    // Eq + Hash: lets Point be used as a HashMap key
    let mut map: HashMap<Point, &str> = HashMap::new();
    map.insert(p1, "first quadrant");
    println!("{:?}", map.get(&Point { x: 3, y: 4 })); // Some("first quadrant")
}"#,
            notes: vec![
                "Always derive Debug — it's free and invaluable when debugging with {:?} or the dbg!() macro.",
                "Copy is implicit (no .clone() call needed); Clone is explicit. A type can be Clone but not Copy (e.g., String).",
                "To use a type as a HashMap key: derive PartialEq + Eq + Hash. The compiler enforces the dependency chain.",
                "If any field doesn't implement a trait (e.g., a raw pointer), you must implement that trait manually.",
                "Default is especially useful with struct update syntax: MyStruct { field: val, ..Default::default() }.",
            ],
        },
        Lesson {
            id: "iterator-trait",
            category: "Traits & Generics",
            title: "The Iterator Trait",
            description: r#"<p>
  The <code>Iterator</code> trait is the beating heart of idiomatic Rust.
  Implement two items and you unlock over 70 adapters for free:
  <code>map</code>, <code>filter</code>, <code>take</code>, <code>flat_map</code>,
  <code>zip</code>, <code>enumerate</code>, <code>collect</code>, and more.
</p>

<h3>The required contract</h3>
<pre><code>trait Iterator {
    type Item;
    fn next(&amp;mut self) -&gt; Option&lt;Self::Item&gt;;
}</code></pre>
<p>
  <code>None</code> signals exhaustion. The behaviour of calling <code>next()</code> again
  after <code>None</code> is unspecified (usually returns <code>None</code> again — "fused").
</p>

<h3>IntoIterator</h3>
<p>
  The <code>for x in collection</code> syntax desugars to a call to
  <code>IntoIterator::into_iter()</code>.
  If your type implements <code>IntoIterator</code>, it works in <code>for</code> loops automatically.
  <code>Iterator</code> implements <code>IntoIterator</code> for itself,
  so any iterator already works in a <code>for</code> loop.
</p>

<h3>TypeScript parallel</h3>
<p>
  This is like implementing the <code>Symbol.iterator</code> protocol —
  once you do, you can use <code>for...of</code>, spread, and all the array methods.
  Rust's iterator adapters are <b>lazy</b> (like generators), unlike JavaScript's
  eager <code>Array.map()</code>.
</p>"#,
            code: r#"struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

// Implementing just type Item + fn next unlocks the entire iterator ecosystem
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None // signals exhaustion
        }
    }
}

fn main() {
    // sum(), min(), max(), count() — all free from Iterator
    let sum: u32 = Counter::new(5).sum();
    println!("sum 1..=5: {}", sum); // 15

    // Lazy adapters — nothing runs until a terminal method drives the chain
    let doubled: Vec<u32> = Counter::new(5)
        .map(|x| x * 2)
        .collect();
    println!("doubled: {:?}", doubled); // [2, 4, 6, 8, 10]

    let evens: Vec<u32> = Counter::new(10)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("evens: {:?}", evens); // [2, 4, 6, 8, 10]

    // zip two iterators together
    let pairs: Vec<(u32, u32)> = Counter::new(3)
        .zip(Counter::new(3).map(|x| x * x))
        .collect();
    println!("pairs: {:?}", pairs); // [(1, 1), (2, 4), (3, 9)]

    // for loop works because Iterator implements IntoIterator for itself
    for n in Counter::new(3) {
        print!("{} ", n);
    }
    println!(); // 1 2 3
}"#,
            notes: vec![
                "Implement Iterator (type Item + fn next) and you get map, filter, collect, zip, enumerate... for free.",
                "Iterator adapters are lazy — no work happens until a terminal method (collect, sum, for) drives the chain.",
                "IntoIterator::into_iter() is what `for x in collection` desugars to. Implement it for your own collections.",
                "The idiomatic trio is iter() (borrows), iter_mut() (mutable borrows), into_iter() (moves/consumes).",
                "Iterator chains compile to tight loops with no heap allocations — they are zero-cost abstractions.",
            ],
        },
        Lesson {
            id: "generics-and-monomorphization",
            category: "Traits & Generics",
            title: "Generics and Monomorphization",
            description: r#"<p>
  Rust generics are a <b>compile-time</b> feature.
  When you call <code>double::&lt;i32&gt;(5)</code> and <code>double::&lt;f64&gt;(2.5)</code>,
  the compiler generates two fully specialised functions — one for each concrete type.
  This process is called <b>monomorphization</b>.
</p>

<h3>Trade-offs</h3>
<ul>
  <li><b>Zero runtime cost</b> — no boxing, no vtable, no dynamic dispatch.
      The generated code is as fast as hand-written type-specific code.</li>
  <li><b>Larger binaries</b> — one copy per unique (function, type) combination.
      Rarely a problem in practice; link-time optimisation (LTO) deduplicates many cases.</li>
  <li><b>Longer compile times</b> — the compiler does more work per instantiation.</li>
</ul>

<h3>vs Java / C# generics</h3>
<p>
  Java erases type parameters at runtime (type erasure); all <code>List&lt;T&gt;</code>
  share one implementation and must box primitives.
  Rust never erases — every combination is fully concrete.
  C# similarly specialises for value types but boxes reference types.
</p>

<h3>vs dyn Trait</h3>
<p>
  <code>dyn Trait</code> is one copy of the code + a vtable.
  Generics are multiple copies + no vtable.
  When in doubt, start with generics; switch to <code>dyn Trait</code>
  when you need a heterogeneous collection or runtime type selection.
</p>

<h3>PhantomData</h3>
<p>
  <code>PhantomData&lt;T&gt;</code> is a zero-sized marker that tells the compiler a struct
  is logically "associated with" type <code>T</code>, even though no field stores a <code>T</code>.
  Common uses: typed IDs, ownership markers, variance annotations.
</p>"#,
            code: r#"use std::ops::Add;
use std::marker::PhantomData;

// Compiler generates: double_i32, double_f64, double_u8, etc. as needed
// Zero runtime cost — no boxing, no vtable
fn double<T: Add<Output = T> + Copy>(x: T) -> T {
    x + x
}

// PhantomData: associate a type parameter with a struct without storing it.
// Zero bytes at runtime — entirely erased by the compiler.
struct TypedId<T> {
    raw: u64,
    _marker: PhantomData<T>,
}

struct User;
struct Order;

impl<T> TypedId<T> {
    fn new(id: u64) -> Self {
        TypedId { raw: id, _marker: PhantomData }
    }
    fn raw(&self) -> u64 { self.raw }
}

fn main() {
    // Each call site triggers a specialised copy — monomorphization in action
    println!("{}", double(5_i32));   // 10  (i32 version compiled)
    println!("{}", double(2.5_f64)); // 5.0 (f64 version compiled)
    println!("{}", double(3_u8));    // 6   (u8 version compiled)

    // TypedId<User> and TypedId<Order> are different types at compile time
    let user_id:  TypedId<User>  = TypedId::new(42);
    let order_id: TypedId<Order> = TypedId::new(99);

    println!("user  id: {}", user_id.raw());
    println!("order id: {}", order_id.raw());

    // The type system prevents accidentally mixing up IDs:
    // let wrong: TypedId<Order> = user_id; // compile error — type mismatch
}"#,
            notes: vec![
                "Monomorphization = zero-cost generics: each concrete instantiation compiles to a dedicated, optimised function.",
                "Binary size grows with unique (function, concrete-type) combinations. Use dyn Trait when code size beats speed.",
                "PhantomData<T> is zero-sized at runtime; it carries compile-time type information only.",
                "PhantomData is also used to control variance (covariance/contravariance) and to express ownership of T.",
                "Unlike Java, Rust never boxes primitives in generics — i32 stays a stack i32 inside a Vec<i32>.",
            ],
        },
    ]
}
