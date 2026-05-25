use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "lifetime-basics",
            category: "Lifetimes",
            title: "Lifetime Annotations",
            description: r#"<p>Every reference in Rust has a <b>lifetime</b> — the scope during which it is valid. The compiler tracks lifetimes to prevent dangling references. Most of the time lifetimes are <b>elided</b> (inferred). You only write them explicitly when the compiler can't figure it out.</p>

<h3>When you need explicit lifetimes</h3>
<ul>
  <li>A function returns a reference and takes <em>multiple</em> reference inputs</li>
  <li>A struct stores a reference field</li>
</ul>

<h3>Reading the syntax</h3>
<p><code>fn longest&lt;'a&gt;(a: &amp;'a str, b: &amp;'a str) -&gt; &amp;'a str</code></p>
<p>Read as: "the returned reference lives at least as long as <em>both</em> <code>a</code> and <code>b</code>". In practice the output lifetime is bounded by the <em>shorter</em> of the two inputs — the compiler enforces this at every call site.</p>

<h3>Struct holding a reference</h3>
<pre><code>struct Excerpt&lt;'a&gt; { text: &amp;'a str }</code></pre>
<p>This says: "an <code>Excerpt</code> cannot outlive the string it borrows from." The struct's lifetime parameter is a promise that the borrowed data will remain valid as long as any <code>Excerpt</code> instance exists.</p>

<h3>'static lifetime</h3>
<p><code>&amp;'static str</code> means the data lives for the entire program. String <em>literals</em> are always <code>'static</code> — they are baked into the binary. Owned values with no borrows (e.g., <code>String</code>) also satisfy <code>'static</code> bounds.</p>

<p>TypeScript comparison: there is no equivalent — JS references are garbage-collected and always valid until collected. Lifetime annotations replace the GC for references that don't own their data.</p>"#,
            code: r##"fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    // Elision rule 3 applies: &self -> output borrows from self.
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {announcement}");
        self.text
    }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        // result must not outlive s2 — compiler enforces this.
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: '{result}'");
    }
    // println!("{result}"); // would not compile — s2 is gone

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().expect("no period found");

    let excerpt = Excerpt { text: first };
    let line = excerpt.announce("breaking news");
    println!("Excerpt: '{line}'");

    // 'static: string literals live for the entire program
    let literal: &'static str = "I am baked into the binary";
    println!("{literal}");
}
"##,
            notes: vec![
                "Lifetime annotations describe relationships between reference durations — they don't change how long data lives.",
                "The returned reference from a multi-input function can only live as long as the shortest input.",
                "Structs storing references need a lifetime parameter so the compiler can enforce the borrow relationship.",
                "'static means the data could live for the program's duration — string literals and owned values qualify.",
                "TypeScript has no equivalent — lifetimes replace garbage collection for borrowed (non-owned) data.",
            ],
        },
        Lesson {
            id: "lifetime-elision",
            category: "Lifetimes",
            title: "When You Don't Have to Write Lifetimes",
            description: r#"<p>The Rust compiler applies three <b>elision rules</b> to infer lifetimes automatically. If all three rules still leave any output lifetime ambiguous, the compiler asks you to annotate explicitly with a clear error message.</p>

<h3>The three elision rules</h3>
<ol>
  <li><b>Each input reference gets its own distinct lifetime.</b><br>
      <code>fn f(x: &amp;str, y: &amp;str)</code> → each gets its own unnamed lifetime.</li>
  <li><b>Exactly one input lifetime? It propagates to all outputs.</b><br>
      <code>fn first_word(s: &amp;str) -&gt; &amp;str</code> → output borrows from <code>s</code>.</li>
  <li><b>One of the inputs is <code>&amp;self</code> or <code>&amp;mut self</code>? Output borrows from self.</b><br>
      Covers most method return types automatically.</li>
</ol>

<h3>When you MUST annotate</h3>
<p>Two or more input references, and the output could come from either one:</p>
<pre><code>// Does not compile — compiler can't tell which input the output comes from:
// fn pick(a: &amp;str, b: &amp;str, flag: bool) -&gt; &amp;str

// Correct — you tell the compiler "output has same lifetime as both inputs":
fn pick&lt;'a&gt;(a: &amp;'a str, b: &amp;'a str, flag: bool) -&gt; &amp;'a str {
    if flag { a } else { b }
}</code></pre>

<h3>Compiler error to recognise</h3>
<pre><code>error[E0106]: missing lifetime specifier
  --&gt; src/main.rs:1:40
   |
   | fn pick(a: &amp;str, b: &amp;str, flag: bool) -&gt; &amp;str
   |            ----     ----                  ^ expected named lifetime parameter</code></pre>

<p>When you see this: count your input references. If there are two or more and the output borrows from them, add <code>'a</code> to both inputs and the output.</p>"#,
            code: r##"// Rule 2: single input reference -> output gets same lifetime (elided).
fn first_word(s: &str) -> &str {
    // Expands to: fn first_word<'a>(s: &'a str) -> &'a str
    s.split_whitespace().next().unwrap_or("")
}

// Rule 2 again: single input.
fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    // 'prefix' gets its own lifetime; output can only come from 's'.
    s.strip_prefix(prefix).unwrap_or(s)
}

// Rule 3: &self method — output borrows from self (elided).
struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn host(&self) -> &str {
        // Expands to: fn host<'a>(&'a self) -> &'a str
        &self.host
    }
}

// EXPLICIT annotation required: output could come from either a or b.
fn pick<'a>(a: &'a str, b: &'a str, prefer_a: bool) -> &'a str {
    if prefer_a { a } else { b }
}

fn main() {
    let sentence = String::from("hello world from Rust");
    println!("first word: '{}'", first_word(&sentence));

    let path = String::from("api/users/42");
    println!("trimmed: '{}'", trim_prefix(&path, "api/"));

    let cfg = Config { host: "localhost".to_string(), port: 8080 };
    println!("host: {}:{}", cfg.host(), cfg.port);

    let a = String::from("alpha");
    let b = String::from("beta");
    println!("pick: '{}'", pick(&a, &b, false));
    println!("pick: '{}'", pick(&a, &b, true));
}
"##,
            notes: vec![
                "Most functions don't need explicit lifetime annotations — elision rules cover the common cases.",
                "The rule of thumb: if your function returns a reference and takes two or more, you likely need 'a.",
                "Structs that store references always need explicit lifetime parameters — elision doesn't apply to struct fields.",
                "Compiler error E0106 'missing lifetime specifier' is the signal to add annotations — the hint is usually correct.",
                "Writing lifetimes explicitly is not a sign of bad code; it's a documentation win for readers of the function signature.",
            ],
        },
        Lesson {
            id: "static-and-borrows",
            category: "Lifetimes",
            title: "'static and Higher-Ranked Trait Bounds (HRTB)",
            description: r#"<p><code>'static</code> doesn't mean "lives forever" in an absolute sense. It means: "this reference is valid for the <em>entire program duration</em> — it has no borrow that could expire before the program ends."</p>

<h3>What satisfies 'static</h3>
<ul>
  <li>String literals: <code>"hello"</code> — stored in the binary's read-only data section</li>
  <li>Owned types with no references: <code>String</code>, <code>Vec&lt;i32&gt;</code>, <code>u64</code> — they own their data</li>
  <li>Explicitly leaked memory: <code>Box::leak(Box::new(x))</code></li>
  <li><b>Not:</b> a reference to a local variable — it expires when the stack frame pops</li>
</ul>

<h3>Higher-Ranked Trait Bounds (HRTB)</h3>
<p>Sometimes a generic bound must hold for <em>any</em> lifetime, not a specific one. That's what <code>for&lt;'a&gt;</code> expresses:</p>
<pre><code>// "For any lifetime 'a, this closure accepts &amp;'a str and returns &amp;'a str."
fn apply&lt;F&gt;(f: F, s: &amp;str) -&gt; &amp;str
where
    F: for&lt;'a&gt; Fn(&amp;'a str) -&gt; &amp;'a str
{ f(s) }</code></pre>

<p>HRTB is rarely written by hand — the compiler infers it automatically in most closure contexts. You'll see it in complex trait objects and iterator combinators. Recognising the syntax in error messages is enough for most senior interviews.</p>

<h3>Practical 'static uses</h3>
<ul>
  <li>Thread spawning: <code>thread::spawn</code> requires <code>F: 'static</code> because the thread might outlive the caller</li>
  <li>Error types: <code>Box&lt;dyn Error + Send + 'static&gt;</code> is the idiomatic dynamic error type</li>
  <li>Global state: <code>static FOO: &amp;str = "..."</code></li>
</ul>

<p>TypeScript comparison: there is no direct equivalent. TypeScript's type system has no lifetime concept — the GC handles reference validity entirely at runtime.</p>"#,
            code: r##"// 'static: the data is valid for the entire program.
static GREETING: &str = "Hello, Rustacean!";

// Only accepts closures that work on 'static strings.
fn shout_static<F: Fn(&'static str) -> String>(f: F) -> String {
    f(GREETING)
}

// HRTB: the closure must work for ANY lifetime 'a, not just 'static.
fn transform<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s).to_uppercase()
}

// Demonstrates the 'static bound on type parameters.
fn needs_static<T: 'static>(_val: T) {
    println!("  T: 'static — value contains no expiring borrows");
}

fn main() {
    // String literals are always 'static.
    let lit: &'static str = "I live for the whole program";
    println!("{lit}");

    // Closure applied to a 'static reference.
    let loud = shout_static(|s| s.to_uppercase());
    println!("{loud}");

    // HRTB in action — |s| s.trim() works for any input lifetime.
    let result = transform(|s| s.trim(), "   padded string   ");
    println!("'{result}'");

    // Owned values satisfy 'static — they contain no borrows.
    println!("\nChecking 'static bounds:");
    needs_static("a literal");      // &'static str ✓
    needs_static(String::from("owned")); // String owns its data ✓
    needs_static(42u64);            // primitives ✓
    // needs_static(&lit);          // &'local T — would NOT compile
}
"##,
            notes: vec![
                "'static means 'no expiring borrows' — owned types like String and Vec satisfy it even though they're not literals.",
                "thread::spawn requires F: 'static + Send because the closure may outlive the spawning scope.",
                "HRTB (for<'a>) is inferred by the compiler in most cases; you'll mainly see it when reading trait object bounds.",
                "Box<dyn Error + Send + 'static> is the idiomatic return type for fallible async functions.",
                "If you see 'borrowed value does not live long enough' — think about whether you need 'static or just a longer scope.",
            ],
        },
        Lesson {
            id: "advanced-generics",
            category: "Lifetimes",
            title: "Advanced Generics: PhantomData and Variance",
            description: r#"<p><code>PhantomData&lt;T&gt;</code> tells the compiler "this type logically contains a <code>T</code>" without actually storing one at runtime. It is zero-sized — no heap or stack overhead.</p>

<h3>Why you need it</h3>
<ul>
  <li><b>Typed IDs</b> — prevent mixing <code>UserId</code> and <code>OrderId</code> at compile time with zero runtime cost</li>
  <li><b>State machine tags</b> — e.g., <code>Connection&lt;Unauthenticated&gt;</code> vs <code>Connection&lt;Authenticated&gt;</code></li>
  <li><b>Drop check</b> — if your type holds a raw pointer to <code>T</code>, <code>PhantomData&lt;T&gt;</code> tells the drop checker it may drop a <code>T</code></li>
  <li><b>Variance control</b> — choose how subtyping flows through your generic type</li>
</ul>

<h3>Variance (brief)</h3>
<p>Rust assigns variance automatically based on how <code>T</code> is used:</p>
<ul>
  <li><b>Covariant</b> (<code>PhantomData&lt;T&gt;</code>): if <code>Dog</code> is a subtype of <code>Animal</code>, <code>Box&lt;Dog&gt;</code> coerces to <code>Box&lt;Animal&gt;</code>. Lifetimes: <code>&amp;'long T</code> coerces to <code>&amp;'short T</code>.</li>
  <li><b>Invariant</b> (<code>PhantomData&lt;*mut T&gt;</code> or <code>&amp;mut T</code>): no substitution — you can't coerce <code>&amp;mut Dog</code> to <code>&amp;mut Animal</code>.</li>
  <li><b>Contravariant</b> (<code>PhantomData&lt;fn(T)&gt;</code>): function argument position.</li>
</ul>
<p>In practice: use <code>PhantomData&lt;T&gt;</code> for owned/covariant, <code>PhantomData&lt;fn(T) -&gt; T&gt;</code> for invariant lifetime markers. The compiler will tell you if you get it wrong.</p>

<p>TypeScript comparison: TypeScript 4.7+ added variance annotations (<code>in T</code>, <code>out T</code>) but they're optional. Rust enforces variance through the type system with no escape hatch.</p>"#,
            code: r##"use std::marker::PhantomData;

// Zero-cost typed ID: u64 under the hood, but UserId and OrderId
// are different types at compile time.
struct Id<T> {
    value:   u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(n: u64) -> Self {
        Id { value: n, _marker: PhantomData }
    }

    fn value(&self) -> u64 {
        self.value
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.value)
    }
}

// Marker types — zero-sized, never instantiated.
struct User;
struct Order;
struct Product;

type UserId   = Id<User>;
type OrderId  = Id<Order>;
type ProductId = Id<Product>;

fn greet_user(id: UserId) {
    println!("Welcome, user {}!", id.value());
}

fn process_order(id: OrderId) {
    println!("Processing order {}...", id.value());
}

fn main() {
    let uid: UserId    = Id::new(1);
    let oid: OrderId   = Id::new(99);
    let pid: ProductId = Id::new(42);

    println!("UserId:    {uid}");
    println!("OrderId:   {oid}");
    println!("ProductId: {pid}");

    greet_user(uid);
    process_order(oid);

    // The lines below would NOT compile:
    // greet_user(oid);      // expected Id<User>, found Id<Order>
    // process_order(uid);   // expected Id<Order>, found Id<User>
    // greet_user(pid);      // expected Id<User>, found Id<Product>

    println!("\nAll type checks passed at compile time — zero runtime cost.");
}
"##,
            notes: vec![
                "PhantomData<T> is zero-sized — it costs nothing at runtime while giving full compile-time type safety.",
                "Typed IDs (Id<User> vs Id<Order>) are the most common PhantomData pattern in production Rust code.",
                "State machine types (Connection<Unauthenticated>) use PhantomData to enforce valid state transitions at compile time.",
                "When holding raw pointers, PhantomData<T> tells the drop checker your type may drop a T — required for soundness.",
                "Variance is usually invisible; it matters when building custom smart pointers or types with raw pointer internals.",
            ],
        },
    ]
}
