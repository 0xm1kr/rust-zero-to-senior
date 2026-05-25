use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        // ── 1. intro ──────────────────────────────────────────────────────────
        Lesson {
            id: "intro",
            category: "Basics",
            title: "Welcome & Setup",
            description: r##"<p>This tutorial targets <b>experienced TypeScript / Node engineers</b> who
want to be interview-ready for a senior Rust role. Every lesson includes
runnable code, dense notes, and constant comparisons back to TypeScript so you
spend time learning Rust concepts — not unlearning programming fundamentals.</p>

<h3>Installing Rust</h3>
<p>Install via <code>rustup</code>, the official toolchain installer and version manager:</p>
<pre>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update        # keep the toolchain current
rustup show          # list installed toolchains and active one</pre>
<p>This installs <code>rustc</code> (compiler), <code>cargo</code> (build tool + package manager),
<code>rustfmt</code> (formatter), and <code>clippy</code> (linter) in one shot.</p>

<h3>Project anatomy</h3>
<ul>
  <li><code>Cargo.toml</code> — manifest: crate name, version, Rust edition, dependencies. Like <code>package.json</code>.</li>
  <li><code>src/main.rs</code> — binary entry point; must contain <code>fn main()</code>.</li>
  <li><code>src/lib.rs</code> — library root (when you're writing a crate for others to depend on).</li>
  <li><code>target/</code> — compiled output. Add to <code>.gitignore</code>; it can be several GB.</li>
  <li><code>Cargo.lock</code> — exact dependency versions. Commit it for binaries; <code>.gitignore</code> it for libraries.</li>
</ul>

<h3>Why Rust?</h3>
<ul>
  <li><b>Memory safety without a GC</b> — no use-after-free, double-free, or null-pointer
  dereferences. Guaranteed at compile time via the ownership system, not at runtime.</li>
  <li><b>Zero-cost abstractions</b> — iterators, closures, and generics are fully compiled away.
  You pay only for what you use; there's no runtime overhead.</li>
  <li><b>Fearless concurrency</b> — the type system prevents data races; you cannot share mutable
  state across threads without explicit synchronization.</li>
  <li><b>Great tooling</b> — <code>cargo build/test/bench/doc/publish</code>, <code>rustfmt</code>,
  <code>clippy</code>, and <code>rust-analyzer</code> (LSP) ship with the language.</li>
  <li><b>Broad ecosystem</b> — systems/embedded, WebAssembly, CLI tools, web servers (Axum, Actix-Web),
  async runtimes (Tokio), databases, ML (Burn, Candle), and more.</li>
</ul>"##,
            code: r##"fn main() {
    // Every Rust binary starts here.
    println!("Hello, Rustacean!");

    // Type inference — no annotation needed when the compiler can figure it out
    let language = "Rust";
    let edition = 2021;
    println!("{language} {edition} edition");

    // Immutable by default; Vec<i32> is a growable heap-allocated list
    let xs: Vec<i32> = vec![1, 2, 3, 4, 5];
    let sum: i32 = xs.iter().sum();
    println!("sum of {xs:?} = {sum}");

    // Zero-cost abstraction: this iterator chain compiles to a tight loop
    let doubled: Vec<i32> = xs.iter().map(|x| x * 2).collect();
    println!("doubled: {doubled:?}");

    println!("\nProject layout:");
    println!("  Cargo.toml  – manifest (name, version, deps)");
    println!("  src/main.rs – binary entry point (this file)");
    println!("  src/lib.rs  – library root (when writing a lib crate)");
    println!("  target/     – build output (add to .gitignore)");
    println!("  Cargo.lock  – exact locked dependency tree");
}
"##,
            notes: vec![
                "Install with `rustup`, not your OS package manager — rustup handles editions and toolchain updates.",
                "`cargo new hello` creates a ready-to-run project; `cargo run` builds and executes in one step.",
                "The Rust edition (2015 / 2018 / 2021) is per-crate and controls minor syntax rules — new projects should use 2021.",
                "Rust has no garbage collector. Memory is managed by the ownership system, which the compiler verifies at compile time.",
                "Zero-cost abstractions: iterator chains, generics, and closures produce the same machine code as hand-written loops.",
            ],
        },

        // ── 2. ts-vs-rust ─────────────────────────────────────────────────────
        Lesson {
            id: "ts-vs-rust",
            category: "Basics",
            title: "Coming from TypeScript: Mental Model",
            description: r##"<p>If you already know TypeScript you understand most Rust <em>concepts</em> —
they just have different names and stricter rules enforced at compile time.
Use this table as a quick translation guide.</p>

<table style="border-collapse:collapse;width:100%;font-size:0.9em">
  <thead>
    <tr style="background:#1e1e2e;color:#cdd6f4">
      <th style="padding:6px 12px;text-align:left;border:1px solid #45475a">TypeScript</th>
      <th style="padding:6px 12px;text-align:left;border:1px solid #45475a">Rust</th>
    </tr>
  </thead>
  <tbody>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>const x = 1</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>let x = 1</code> — immutable binding</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>let x = 1</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>let mut x = 1</code></td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>const MAX = 100</code> (module-level)</td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>const MAX: u32 = 100;</code> — compile-time evaluated, must be typed</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>null</code> / <code>undefined</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>Option&lt;T&gt;</code>: <code>Some(value)</code> or <code>None</code></td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>throw new Error("...")</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>Err(e)</code> + <code>Result&lt;T, E&gt;</code> + <code>?</code></td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>class Foo { ... }</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>struct Foo { ... }</code> + <code>impl Foo { ... }</code></td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>interface</code> / <code>type</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>trait</code> (behaviour) + generics + type aliases</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>async/await</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>async/await</code> — same syntax, but needs an executor (Tokio, async-std)</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>any</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>Box&lt;dyn Any&gt;</code> — and you almost never need it</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>T[]</code> / <code>Array&lt;T&gt;</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>Vec&lt;T&gt;</code> (heap, growable) / <code>[T; N]</code> (stack, fixed) / <code>&amp;[T]</code> (slice)</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>{ key: T }</code> objects</td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>struct</code> for known shapes; <code>HashMap&lt;K, V&gt;</code> for dynamic maps</td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>npm</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>cargo</code></td></tr>
    <tr><td style="padding:5px 12px;border:1px solid #45475a"><code>tsc</code> / <code>eslint</code> / <code>prettier</code></td>
        <td style="padding:5px 12px;border:1px solid #45475a"><code>rustc</code> / <code>clippy</code> / <code>rustfmt</code></td></tr>
  </tbody>
</table>

<h3>The three things TS devs take longest to internalize</h3>
<ol>
  <li><b>Ownership &amp; borrowing</b> — every value has exactly one owner at a time.
  Moving a value transfers ownership; the original binding becomes invalid.
  References let you <em>borrow</em> without taking ownership.
  The compiler tracks all of this statically — there is no runtime check.</li>
  <li><b>The compiler is your pair programmer</b> — Rust's borrow checker rejects programs
  with memory hazards. Don't try to fight it. Read the error message, trust it, and
  restructure your code. It's almost always right.</li>
  <li><b>Lifetimes are about code regions, not clock time</b> — a lifetime annotation
  says "this reference must remain valid for at least this region of code". It has
  nothing to do with milliseconds or scope depth; it's about proving to the compiler
  that a reference won't outlive the data it points to.</li>
</ol>"##,
            code: r##"use std::collections::HashMap;

fn main() {
    // --- let vs let mut ---
    let x = 42;           // immutable (TS: const x = 42)
    let mut count = 0;    // mutable   (TS: let count = 0)
    count += 1;
    println!("x={x}, count={count}");

    // --- Option<T> instead of null/undefined ---
    let maybe: Option<&str> = Some("hello");
    let nothing: Option<&str> = None;
    println!("maybe={maybe:?}, nothing={nothing:?}");

    // --- HashMap instead of plain object ---
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("alice", 95);
    scores.insert("bob", 87);
    println!("alice: {}", scores["alice"]);

    // --- struct + impl instead of class ---
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
    let p = Point { x: 3.0, y: 4.0 };
    println!("distance = {}", p.distance_from_origin()); // 5.0

    // --- Result<T, E> instead of throw ---
    fn parse_positive(s: &str) -> Result<u32, String> {
        s.parse::<u32>().map_err(|e| e.to_string())
    }
    match parse_positive("42") {
        Ok(n)  => println!("parsed: {n}"),
        Err(e) => println!("error: {e}"),
    }
    match parse_positive("oops") {
        Ok(n)  => println!("parsed: {n}"),
        Err(e) => println!("error: {e}"),
    }
}
"##,
            notes: vec![
                "In Rust, `let` is immutable — closer to TS `const` than TS `let`. Use `let mut` when you need to reassign.",
                "`Option<T>` and `Result<T, E>` are enums, not special language keywords. They're just types in std.",
                "There's no `class` keyword. Data (`struct`) and behaviour (`impl`) are always separate.",
                "`async/await` looks the same as TS but Rust has no built-in executor — you must choose one (Tokio is the most common).",
                "Ownership is the one concept with no direct TypeScript analogue. Master it first; everything else follows.",
            ],
        },

        // ── 3. variables ──────────────────────────────────────────────────────
        Lesson {
            id: "variables",
            category: "Basics",
            title: "Variables & Mutability",
            description: r##"<p>In Rust, <b>all bindings are immutable by default</b>. This isn't just a
style convention (like preferring <code>const</code> in TypeScript) — the
compiler enforces it. Immutability-by-default eliminates a large class of
accidental-mutation bugs without any runtime cost.</p>

<h3>let vs let mut</h3>
<ul>
  <li><code>let x = …</code> — immutable binding. You cannot reassign <code>x</code> after this line.
  Analogous to TypeScript's <code>const</code>.</li>
  <li><code>let mut x = …</code> — mutable binding. You can reassign and mutate.
  Analogous to TypeScript's <code>let</code>.</li>
  <li>There is <b>no <code>var</code></b> in Rust.</li>
</ul>

<h3>Shadowing</h3>
<p>You can redeclare a variable with the same name in the same scope using a
second <code>let</code>. Unlike mutation, shadowing <b>can change the type</b> of the
binding. It's idiomatic for stepwise transformations (parse a string into a
number, or trim whitespace) without inventing new names like
<code>raw_input</code> vs <code>parsed_input</code>.</p>

<h3>const</h3>
<p><code>const NAME: T = …;</code> — a compile-time constant. The value is
inlined wherever it is used; there is no runtime memory allocation for it.
Must always have a type annotation. Can live in any scope (module, function,
block).</p>

<h3>static</h3>
<p><code>static NAME: T = …;</code> — a fixed memory location that lives for
the entire program. Used for string literals, lookup tables, and (with care)
shared globals. <code>static mut</code> exists but requires <code>unsafe</code>;
prefer <code>std::sync::atomic</code> or <code>Mutex</code> instead.</p>"##,
            code: r##"fn main() {
    // Immutable binding — the compiler prevents reassignment
    let x = 5;
    println!("x = {x}");
    // x = 6; // compile error: cannot assign twice to immutable variable

    // Mutable binding
    let mut count = 0;
    count += 1;
    count += 1;
    println!("count = {count}");

    // Shadowing — reuse the name, optionally with a different type
    let spaces = "   ";            // &str
    let spaces = spaces.len();     // now usize — shadowing changed the type!
    println!("spaces = {spaces}");

    // Shadowing in a nested block is local to that block
    let y = 10;
    let y = {
        let y = y * 2; // shadows outer y inside this block
        y + 1          // block evaluates to 21
    };
    println!("y = {y}"); // 21

    // const — compile-time, always typed, inlined at use sites
    const MAX_POINTS: u32 = 100_000; // underscores improve readability
    println!("MAX_POINTS = {MAX_POINTS}");

    // static — single memory address, lives for the whole program
    static GREETING: &str = "Hello";
    println!("{GREETING}, world!");

    // Numeric literal suffixes and underscores
    let million = 1_000_000_u64;
    let pi = 3.141_592_653_589_793_f64;
    println!("million = {million}, pi ≈ {pi:.5}");
}
"##,
            notes: vec![
                "`let` is immutable — the compiler will reject any attempt to reassign or mutate it.",
                "`let mut` is not the default. You must opt in to mutability; this makes mutable state easy to spot in a code review.",
                "Shadowing with `let` creates a *new* binding. The old value is dropped (or just shadowed). It can change the type.",
                "`const` values are inlined — no heap allocation, no runtime load. Use for mathematical constants, limits, config.",
                "`static` has a stable address. Use for read-only string literals (`&'static str`) and thread-safe shared globals.",
                "Numeric separators (`1_000_000`) are purely cosmetic and compile away completely.",
            ],
        },

        // ── 4. types ──────────────────────────────────────────────────────────
        Lesson {
            id: "types",
            category: "Basics",
            title: "Basic Types & Conversions",
            description: r##"<p>Rust is <b>strongly and statically typed</b>. The compiler infers most types
from context, but you can always annotate explicitly. There is
<b>no implicit numeric coercion</b> — every conversion between numeric types
must be written out.</p>

<h3>Integer types</h3>
<p>Signed: <code>i8</code>, <code>i16</code>, <code>i32</code>, <code>i64</code>,
<code>i128</code>, <code>isize</code> (pointer-sized). Unsigned: same with
<code>u</code> prefix. <b>Default integer type is <code>i32</code></b>.
<code>usize</code> / <code>isize</code> match the platform's pointer width and
are used for indexing and slice lengths.</p>

<h3>Float types</h3>
<p><code>f32</code> and <code>f64</code>. <b>Default is <code>f64</code></b>
(same precision as JavaScript's <code>number</code>). Prefer <code>f64</code>
unless you have a specific memory / SIMD reason to use <code>f32</code>.</p>

<h3>bool and char</h3>
<ul>
  <li><code>bool</code>: <code>true</code> / <code>false</code>. No truthy/falsy coercion — conditions must be explicitly <code>bool</code>.</li>
  <li><code>char</code>: a <b>4-byte Unicode scalar value</b>, not a byte. <code>'a'</code> and <code>'🦀'</code> are both valid <code>char</code>. Use single quotes.</li>
</ul>

<h3>&amp;str vs String</h3>
<ul>
  <li><code>&amp;str</code> — a borrowed string <em>slice</em> (fat pointer: data + length).
  Usually points into read-only memory (string literals) or into a <code>String</code>.
  You don't own it.</li>
  <li><code>String</code> — owned, heap-allocated, growable UTF-8 string.
  Convert: <code>"hello".to_owned()</code> or <code>String::from("hello")</code>.</li>
  <li><b><code>s.len()</code> returns bytes, not characters.</b> Use
  <code>s.chars().count()</code> for Unicode character count. This matters for any
  non-ASCII text.</li>
</ul>

<h3>Numeric conversions</h3>
<p>Rust never coerces numeric types silently. Three options:</p>
<ul>
  <li><code>x as T</code> — C-style cast; potentially lossy (truncates, wraps).</li>
  <li><code>T::from(x)</code> / <code>x.into()</code> — infallible; only compiles when
  the conversion is lossless (e.g. <code>i32</code> → <code>i64</code>).</li>
  <li><code>T::try_from(x)</code> / <code>x.try_into()</code> — returns
  <code>Result</code>; use when the value might not fit.</li>
</ul>"##,
            code: r##"use std::convert::TryInto;

fn main() {
    // --- Integer types ---
    let a: i32  = 42;
    let b: u8   = 255;
    let c: usize = 7; // indexing always uses usize
    println!("a={a}, b={b}, c={c}");

    // Overflow is a compile-error for literals, a panic in debug builds
    // let overflow: u8 = 256; // compile error

    // --- Float types ---
    let f: f64 = 3.141_592_653_589_793;
    println!("pi ≈ {f:.4}");

    // --- bool (no truthy/falsy; must be explicitly bool) ---
    let flag = true;
    if flag {
        println!("flag is true — no implicit coercion");
    }

    // --- char is 4 bytes / Unicode scalar value ---
    let letter: char = 'A';
    let crab: char = '🦀';
    println!("char size: {} bytes; letter={letter}, crab={crab}",
        std::mem::size_of::<char>());

    // --- Explicit numeric conversions ---
    let x: i32 = 300;
    let y: f64 = x as f64;     // as cast — always works, may lose precision
    let z: i64 = i64::from(x); // From — infallible widening
    // TryInto returns Result; 300 does NOT fit in u8, so we get Err here.
    let w: Result<u8, _> = x.try_into();
    println!("x={x} → u8 try_into: {:?}", w.err().map(|e| e.to_string()));
    let small: u8 = 42_i32.try_into().unwrap(); // fits — Ok(42)
    println!("x={x}, y={y}, z={z}, small={small}");

    // --- &str vs String ---
    let s: &str   = "hello";          // string slice — borrowed
    let owned: String = s.to_owned(); // heap-allocated, owned copy
    let back: &str = &owned;          // borrow a slice of the String
    println!("s={s}, owned={owned}, back={back}");

    // len() = bytes; chars().count() = Unicode scalar values
    let emoji = "🦀🦀🦀";
    println!("'{}' → {} bytes, {} chars", emoji, emoji.len(), emoji.chars().count());
}
"##,
            notes: vec![
                "Default integer is `i32`; default float is `f64` (same as JavaScript's `number`).",
                "`usize` is the type for indexing slices and Vecs — it matches the platform's pointer width.",
                "`char` is 4 bytes (Unicode scalar). A `String` is UTF-8 bytes, so `s.len()` gives bytes, not chars.",
                "No implicit numeric coercion. Use `as` for lossy casts, `From`/`Into` for infallible widening, `TryFrom`/`TryInto` for checked narrowing.",
                "`&str` is a borrowed view into UTF-8 data; `String` owns the data on the heap. Most functions accept `&str` so they work with both.",
                "Integer overflow panics in debug builds and wraps in release. Use `.wrapping_add()`, `.checked_add()`, or `.saturating_add()` for explicit control.",
            ],
        },

        // ── 5. constants ──────────────────────────────────────────────────────
        Lesson {
            id: "constants",
            category: "Basics",
            title: "Constants and Static",
            description: r##"<p>Rust has two mechanisms for "compile-time fixed" named values:
<code>const</code> and <code>static</code>. Both require explicit type
annotations — there is no inference for either.</p>

<h3>const</h3>
<p><code>const NAME: T = expr;</code> — the expression is evaluated at compile
time and the result is <b>inlined at every use site</b>. There is no memory
address; you cannot take a reference to a <code>const</code> (well, you can,
but the compiler may create a temporary). Analogous to C++ <code>constexpr</code>
or TypeScript's module-level <code>const</code> — but with actual
compile-time evaluation of arithmetic.</p>

<h3>static</h3>
<p><code>static NAME: T = expr;</code> — allocates exactly <b>one memory
location</b> that persists for the entire program lifetime
(<code>'static</code> lifetime). You can take a reference to it:
<code>&amp;NAME</code> has type <code>&amp;'static T</code>. Useful for
lookup tables, interned strings, and (with care) shared globals.</p>
<p><code>static mut</code> is technically valid but requires <code>unsafe</code>
to access. Instead, prefer <code>std::sync::atomic</code> types for counters
and <code>std::sync::Mutex&lt;T&gt;</code> for general mutable globals.</p>

<h3>Enums as named constants</h3>
<p>Rather than a set of <code>const NORTH: u8 = 0</code> magic numbers, define
an <code>enum</code>. The compiler ensures exhaustive handling in
<code>match</code>, gives you meaningful names in debug output, and imposes
zero runtime overhead. This is idiomatic Rust and a preview of the much
richer pattern-matching you'll use throughout the language.</p>

<h3>const in any scope</h3>
<p>Constants can be declared inside functions and blocks — not just at module
level. This is useful for algorithm-local magic values that don't belong in the
module's public API.</p>"##,
            code: r##"// Constants at module scope (outside any function)
const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR:   u64 = SECONDS_PER_MINUTE * 60;   // computed at compile time
const SECONDS_PER_DAY:    u64 = SECONDS_PER_HOUR   * 24;

// static — one memory address, 'static lifetime
static APP_NAME: &str = "RustTutor";

// Enum as named constants: zero overhead, exhaustiveness-checked
#[derive(Debug, Clone, Copy, PartialEq)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

fn describe(d: CardinalDirection) -> &'static str {
    match d {
        CardinalDirection::North => "heading north ↑",
        CardinalDirection::South => "heading south ↓",
        CardinalDirection::East  => "heading east →",
        CardinalDirection::West  => "heading west ←",
    }
}

fn main() {
    println!("{APP_NAME}");
    println!("1 day = {SECONDS_PER_DAY} seconds");

    for dir in [
        CardinalDirection::North,
        CardinalDirection::East,
        CardinalDirection::South,
        CardinalDirection::West,
    ] {
        println!("{dir:?}: {}", describe(dir));
    }

    // const inside a function — scoped to this block, not visible outside
    const LOCAL_LIMIT: usize = 50;
    println!("local limit = {LOCAL_LIMIT}");

    // Atomic instead of `static mut` — safe, no `unsafe` needed
    use std::sync::atomic::{AtomicU32, Ordering};
    static REQUEST_COUNT: AtomicU32 = AtomicU32::new(0);
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("requests handled: {}", REQUEST_COUNT.load(Ordering::Relaxed));
}
"##,
            notes: vec![
                "`const` is compile-time only — no memory address, the value is substituted at every use site (like a C `#define`, but typed and scoped).",
                "`static` has a stable address and `'static` lifetime. You can hold `&'static T` references to statics anywhere.",
                "Both `const` and `static` require explicit type annotations — the compiler does not infer them.",
                "`static mut` exists but requires `unsafe` to read or write. Use `AtomicT` types for counters and `Mutex<T>` for everything else.",
                "Enums beat `const` integer groups: the compiler enforces exhaustive handling in `match` and you get meaningful names in debug output.",
                "Constants can be defined inside any scope (function, block, impl block), not just at the top of a module.",
            ],
        },
    ]
}
