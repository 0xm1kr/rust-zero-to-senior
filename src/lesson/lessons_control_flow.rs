use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        // ── 1. if-loops ───────────────────────────────────────────────────────
        Lesson {
            id: "if-loops",
            category: "Control Flow",
            title: "if, loop, while, for",
            description: r##"<p>Rust's control flow constructs will feel familiar from TypeScript — with
one important upgrade: <b><code>if</code>, <code>loop</code>, and
<code>match</code> are expressions</b>, not just statements. They produce
values, so you can assign their results directly.</p>

<h3>if</h3>
<p>No parentheses around the condition (unlike JS/TS). Both branches of an
<code>if</code> expression must produce the same type. Use it anywhere a value
is expected — the ternary operator doesn't exist in Rust; <code>if</code>
replaces it.</p>

<h3>loop</h3>
<p><code>loop { … }</code> runs forever until a <code>break</code>. Crucially,
<code>break value</code> causes the <code>loop</code> expression to evaluate to
<code>value</code>. This is handy for retry logic and state machines where you
need to return a result from the loop body. Loop labels
(<code>'outer: loop</code>) let you <code>break</code> or <code>continue</code>
a specific outer loop from inside a nested loop.</p>

<h3>while</h3>
<p>Standard conditional loop. <code>while let Some(x) = iter.next()</code> is
a common idiom for draining an iterator (though <code>for</code> is usually
preferred).</p>

<h3>for</h3>
<p><code>for x in expr</code> consumes any type that implements
<code>IntoIterator</code>. Ranges (<code>0..10</code> exclusive,
<code>0..=10</code> inclusive), <code>Vec</code>, arrays, slices, and many
stdlib types all implement it. <code>vec.iter()</code> yields
<code>&amp;T</code> (shared references); <code>vec.iter_mut()</code> yields
<code>&amp;mut T</code>; <code>vec.into_iter()</code> yields owned
<code>T</code> and consumes the Vec.</p>"##,
            code: r##"fn main() {
    // if is an expression — assign its result directly
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {label}");

    // Nested if-else (replaces TS ternary chains)
    let grade = if n >= 90 { "A" } else if n >= 75 { "B" } else { "C" };
    println!("grade: {grade}");

    // loop returns a value via `break value`
    let mut attempt = 0;
    let found = loop {
        attempt += 1;
        if attempt * attempt >= 50 {
            break attempt; // the loop expression evaluates to `attempt`
        }
    };
    println!("first n where n² ≥ 50: {found} ({}²={})", found, found * found);

    // Loop labels — break out of an outer loop from inside an inner one
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 6 {
                println!("breaking outer at i={i}, j={j}");
                break 'outer;
            }
        }
    }

    // while — standard conditional loop
    let mut x = 1_u64;
    while x < 1_000 {
        x *= 2;
    }
    println!("first power of 2 ≥ 1000: {x}");

    // for over a range (exclusive end)
    let mut sum = 0;
    for i in 0..10 {
        sum += i;
    }
    println!("sum 0..10 = {sum}");

    // for over a Vec with iter() — yields &T
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("  - {fruit}");
    }

    // enumerate gives (index, &T)
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  [{i}] {fruit}");
    }
}
"##,
            notes: vec![
                "`if` is an expression — both branches must have the same type. This replaces the ternary operator (`? :`).",
                "`loop { break value; }` is the idiomatic way to retry an operation until success and return the result.",
                "Loop labels (`'name:`) let you `break` or `continue` a specific enclosing loop — no need for flag variables.",
                "`0..10` is exclusive (0–9); `0..=10` is inclusive (0–10). Both are `Range` types implementing `IntoIterator`.",
                "`.iter()` borrows elements as `&T`; `.iter_mut()` as `&mut T`; `.into_iter()` takes ownership and yields `T`.",
                "There is no `do…while` in Rust; use `loop { …; if !cond { break; } }` instead.",
            ],
        },

        // ── 2. match ──────────────────────────────────────────────────────────
        Lesson {
            id: "match",
            category: "Control Flow",
            title: "match and Patterns",
            description: r##"<p><code>match</code> is Rust's most powerful control-flow construct. Think of
it as a <code>switch</code> that has been carefully designed to be impossible
to misuse: it is <b>exhaustive</b> (the compiler rejects unhandled cases),
expression-based (it returns a value), and works on any type including
structs, enums, tuples, and slices.</p>

<h3>Pattern forms</h3>
<ul>
  <li><b>Literal:</b> <code>0</code>, <code>'a'</code>, <code>true</code></li>
  <li><b>Range (inclusive):</b> <code>1..=5</code></li>
  <li><b>Or-pattern:</b> <code>1 | 2 | 3</code></li>
  <li><b>Wildcard:</b> <code>_</code> — matches anything, binds nothing</li>
  <li><b>Binding:</b> <code>x</code> — matches anything and binds it to <code>x</code></li>
  <li><b>Binding with constraint:</b> <code>x @ 1..=9</code> — match the range <em>and</em> bind the value</li>
  <li><b>Guard:</b> <code>x if x &gt; 100</code> — extra boolean condition after the pattern</li>
  <li><b>Destructuring tuple:</b> <code>(a, b, _)</code></li>
  <li><b>Destructuring struct:</b> <code>Point { x, y }</code></li>
  <li><b>Enum variant:</b> <code>Some(v)</code>, <code>Ok(n)</code>, <code>Err(e)</code></li>
</ul>

<p>Arms are evaluated in <b>top-to-bottom order</b>; the first matching arm
wins. Arms are expressions separated by commas. Block arms (<code>{ … }</code>)
don't need a trailing comma.</p>

<h3>Exhaustiveness</h3>
<p>If your <code>match</code> doesn't cover every possible value the compiler
will refuse to compile. Use <code>_</code> or a named catch-all as the final
arm when you genuinely want a default. This means adding a new variant to an
enum <em>immediately</em> surfaces every <code>match</code> in the codebase
that needs updating — a powerful refactoring aid.</p>"##,
            code: r##"fn http_category(status: u16) -> &'static str {
    match status {
        100..=199 => "Informational",
        200..=299 => "Success",
        300..=399 => "Redirection",
        400..=499 => "Client Error",
        500..=599 => "Server Error",
        _         => "Unknown",
    }
}

fn describe_number(n: i32) -> String {
    match n {
        0           => "zero".to_string(),
        1 | 2 | 3   => "one, two, or three".to_string(),
        x @ 4..=9   => format!("small: {x}"),       // bind with @
        x if x < 0  => format!("negative: {x}"),    // guard
        x           => format!("large: {x}"),
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // variant carrying data
}

fn cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny           => 1,
        Coin::Nickel          => 5,
        Coin::Dime            => 10,
        Coin::Quarter(state)  => {
            println!("  Quarter from {state}!");
            25
        }
    }
}

fn main() {
    // Range patterns
    for code in [200_u16, 301, 404, 500, 999] {
        println!("{code} → {}", http_category(code));
    }
    println!();

    // Literal, or-pattern, @, and guard
    for n in [-5, 0, 2, 7, 42] {
        println!("{n:>4} → {}", describe_number(n));
    }
    println!();

    // Enum variant with data
    let coins = vec![
        Coin::Quarter("Alaska".to_string()),
        Coin::Dime,
        Coin::Nickel,
        Coin::Penny,
    ];
    let total: u32 = coins.iter().map(cents).sum();
    println!("total: {total}¢");

    // Destructuring a tuple
    let pair = (true, 42_i32);
    let msg = match pair {
        (true,  n) if n > 0 => format!("positive true: {n}"),
        (true,  _)          => "true but not positive".to_string(),
        (false, n)          => format!("false: {n}"),
    };
    println!("{msg}");
}
"##,
            notes: vec![
                "`match` is exhaustive — the compiler rejects incomplete patterns. Use `_` as a catch-all only when you genuinely want a default.",
                "Match arms are expressions; no `break` needed (unlike C's `switch`).",
                "Guards (`if condition` after a pattern) are checked after the pattern matches. If the guard fails, the next arm is tried.",
                "`x @ 1..=9` binds the matched value to `x` *and* checks the range — useful when you need the value and want to constrain it.",
                "Or-patterns (`1 | 2 | 3`) collapse multiple arms into one — cleaner than a chain of `||` in a guard.",
                "Adding a variant to an enum breaks every `match` that lacks a wildcard arm — the compiler tells you exactly where to update.",
            ],
        },

        // ── 3. functions ──────────────────────────────────────────────────────
        Lesson {
            id: "functions",
            category: "Control Flow",
            title: "Functions & Return Values",
            description: r##"<p>Functions are declared with <code>fn</code>. <b>Parameter and return types
are always explicit</b> — Rust does not infer function signatures (by design;
it keeps code readable without IDE help and makes API contracts clear).</p>

<h3>Implicit return</h3>
<p>The <b>last expression without a semicolon</b> is the implicit return value.
Adding a semicolon turns it into a statement that produces <code>()</code>
(unit). Forgetting to remove a semicolon from the final expression is a
common beginner error that the compiler catches with a "mismatched types"
message.</p>

<h3>Unit type ()</h3>
<p>Functions without a <code>-&gt; Type</code> annotation implicitly return
<code>()</code>, Rust's equivalent of <code>void</code>. Unlike <code>void</code>,
<code>()</code> is a real zero-size type — you can store it in a variable, put
it in a tuple, or return it from a closure.</p>

<h3>Never type !</h3>
<p>Functions that never return have type <code>!</code> (the "never" type).
Examples: <code>panic!()</code>, <code>std::process::exit()</code>, and
infinite loops. <code>!</code> coerces to any type, which is why
<code>panic!()</code> can legally appear in any <code>match</code> arm
regardless of what type the arm must produce.</p>

<h3>Multiple return values</h3>
<p>Return a <b>tuple</b>: <code>fn divmod(a: i32, b: i32) -&gt; (i32, i32)</code>.
Destructure at the call site: <code>let (q, r) = divmod(17, 5);</code>. No
special syntax needed; tuples are just values.</p>

<h3>Functions as values</h3>
<p>Function pointers have the type <code>fn(T, U) -&gt; V</code>. You can
store them in variables, pass them to other functions, and collect them in
arrays. For closures (which capture environment) use the
<code>Fn</code>/<code>FnMut</code>/<code>FnOnce</code> traits instead.</p>"##,
            code: r##"// Last expression without `;` is the return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon — this is the return value
}

// Multiple return values via tuple
fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// Unit return — `-> ()` is implicit when omitted
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// Early return with explicit `return`
fn first_positive(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n > 0 {
            return Some(n); // explicit early return
        }
    }
    None // implicit return at end — no semicolon
}

// Generic function — works for any type implementing Display + PartialOrd
fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn main() {
    println!("add(3, 4) = {}", add(3, 4));

    let (q, r) = divmod(17, 5);
    println!("17 / 5 = {q} remainder {r}");

    greet("Rustacean");

    let nums = [-3, -1, 4, 1, 5];
    match first_positive(&nums) {
        Some(n) => println!("first positive: {n}"),
        None    => println!("no positive numbers"),
    }

    println!("clamp(150, 0, 100) = {}", clamp(150, 0, 100));
    println!("clamp(-5,  0, 100) = {}", clamp(-5, 0, 100));

    // Functions are first-class values
    let op: fn(i32, i32) -> i32 = add;
    let ops: [fn(i32, i32) -> i32; 2] = [add, |a, b| a - b];
    for f in &ops {
        println!("f(10, 3) = {}", f(10, 3));
    }
    let _ = op;
}
"##,
            notes: vec![
                "The last expression without a semicolon is the implicit return value. Adding `;` makes it a statement returning `()`.",
                "Use `return` for early exits; use the implicit return for the normal result at the end of the function.",
                "`()` is a real zero-size type, not a keyword. Functions that don't return a meaningful value return `()`.",
                "`!` (never type) coerces to any type, which is why `panic!()` can appear in any branch of a `match` expression.",
                "Multiple return values are just tuples. Destructure them at the call site: `let (a, b) = f();`.",
                "Function pointer type syntax is `fn(T) -> U`. Closures use the `Fn`/`FnMut`/`FnOnce` traits instead.",
            ],
        },

        // ── 4. closures ───────────────────────────────────────────────────────
        Lesson {
            id: "closures",
            category: "Control Flow",
            title: "Closures and Iterators",
            description: r##"<p>Closures in Rust work like arrow functions in TypeScript — compact inline
functions that capture their enclosing environment. Unlike TS, Rust
distinguishes three closure <em>traits</em> based on how the closure uses its
captured variables. The compiler picks the most permissive trait that fits.</p>

<h3>The three closure traits</h3>
<ul>
  <li><code>Fn</code> — borrows captured variables <b>immutably</b>; callable
  any number of times. Most closures fall here.</li>
  <li><code>FnMut</code> — borrows captured variables <b>mutably</b>; callable
  any number of times but requires exclusive access.</li>
  <li><code>FnOnce</code> — <b>takes ownership</b> of captured variables; can
  only be called once. Every closure implements at least <code>FnOnce</code>.
  <code>move ||</code> closures are often <code>FnOnce</code>.</li>
</ul>
<p>The hierarchy is <code>Fn ⊆ FnMut ⊆ FnOnce</code>. Accepting
<code>impl Fn</code> in a parameter also accepts <code>FnMut</code> and
<code>FnOnce</code>.</p>

<h3>move closures</h3>
<p><code>move || …</code> forces the closure to take <em>ownership</em> of all
captured variables. Required when the closure outlives the current scope — for
example, when spawning a thread or returning a closure from a function.</p>

<h3>Iterators</h3>
<p>Rust iterators are <b>lazy</b> — adapters like <code>.map()</code> and
<code>.filter()</code> build a pipeline description but do no work until the
iterator is consumed by <code>.collect()</code>, <code>.sum()</code>,
<code>.for_each()</code>, or a <code>for</code> loop. The compiler typically
inlines the entire chain into a single loop with zero intermediate heap
allocations.</p>
<p>Common adapters: <code>.map</code>, <code>.filter</code>, <code>.flat_map</code>,
<code>.take</code>, <code>.skip</code>, <code>.zip</code>, <code>.enumerate</code>,
<code>.chain</code>, <code>.peekable</code>. Terminal consumers:
<code>.collect()</code>, <code>.sum()</code>, <code>.fold()</code>,
<code>.count()</code>, <code>.any()</code>, <code>.all()</code>,
<code>.find()</code>, <code>.max()</code>.</p>"##,
            code: r##"fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    // Basic closure — captures `factor` by reference (Fn)
    let factor = 3;
    let triple = |x: i32| x * factor;
    println!("triple(5) = {}", apply(triple, 5));

    // FnMut — mutably borrows `count`
    let mut count = 0;
    let mut inc = || {
        count += 1;
        count
    };
    println!("inc() = {}", inc()); // 1
    println!("inc() = {}", inc()); // 2
    drop(inc); // drop the closure to release the mutable borrow
    println!("count after closure dropped = {count}");

    // move closure — takes ownership of `greeting`
    let greeting = String::from("hello");
    let say = move || println!("{greeting}!"); // greeting is moved in
    say();
    // println!("{greeting}"); // compile error: greeting was moved

    // Iterator pipeline: sum of squares of even numbers in 1..=10
    // (1..=10) yields i32 owned values; filter sees &i32, so use |&x|
    let sum_sq_evens: i32 = (1_i32..=10)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("sum of squares of evens 1..=10 = {sum_sq_evens}");
    // 2²+4²+6²+8²+10² = 4+16+36+64+100 = 220

    // fold is reduce — compute 5!
    let factorial: u64 = (1_u64..=5).fold(1, |acc, x| acc * x);
    println!("5! = {factorial}");

    // Lazy: infinite range filtered and taken — no infinite loop
    let first_five_mult3: Vec<i32> = (0_i32..)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect();
    println!("first 5 multiples of 3: {first_five_mult3:?}");

    // collect into different container types
    let words = vec!["hello", "world", "rust"];
    let upper: Vec<String> = words.iter().map(|s| s.to_uppercase()).collect();
    println!("{upper:?}");
}
"##,
            notes: vec![
                "`Fn` borrows immutably, `FnMut` borrows mutably, `FnOnce` takes ownership. The compiler picks the most general trait that fits.",
                "`move ||` forces ownership of all captured variables — required for closures that outlive the current scope (threads, `'static` bounds).",
                "Iterator adapters are lazy — they build a description of the pipeline. Nothing runs until you call a consumer like `.collect()` or `.sum()`.",
                "Prefer iterators over manual `for` loops for transformations: they're often faster (no bounds checks) and more composable.",
                "`.collect::<Vec<_>>()` is idiomatic. The `_` tells the compiler to infer the element type from context.",
                "Infinite iterators (`.filter(…).take(n)`) are perfectly fine — laziness means only the first `n` elements are computed.",
            ],
        },

        // ── 5. options-and-questionmark ───────────────────────────────────────
        Lesson {
            id: "options-and-questionmark",
            category: "Control Flow",
            title: "Option<T> and the ? operator",
            description: r##"<p><code>Option&lt;T&gt;</code> is Rust's answer to <code>null</code> and
<code>undefined</code>. There are no null pointer exceptions — if a function
can return nothing, its return type says so, and the compiler forces you to
handle both cases.</p>

<pre>enum Option&lt;T&gt; {
    Some(T),   // a value is present
    None,      // no value
}</pre>

<h3>Constructing and consuming</h3>
<ul>
  <li><code>Some(value)</code> — wrap a value</li>
  <li><code>None</code> — absence</li>
  <li><code>match</code> — exhaustive handling (most explicit)</li>
  <li><code>if let Some(x) = opt { … }</code> — handle only <code>Some</code></li>
  <li><code>.unwrap()</code> — panics on <code>None</code>; prototype/test only</li>
  <li><code>.expect("msg")</code> — panics with a message; slightly better than <code>unwrap</code></li>
  <li><code>.unwrap_or(default)</code> — safe fallback value</li>
  <li><code>.unwrap_or_else(|| compute())</code> — lazy fallback (only evaluated if <code>None</code>)</li>
  <li><code>.map(|x| …)</code> — transform the inner value; <code>None</code> passes through</li>
  <li><code>.and_then(|x| …)</code> — flatMap; your closure returns an <code>Option</code></li>
  <li><code>.filter(|x| predicate)</code> — <code>None</code> if predicate fails</li>
  <li><code>.or(other_opt)</code> — use <code>other_opt</code> if <code>None</code></li>
</ul>

<h3>The ? operator</h3>
<p>Inside a function that returns <code>Option&lt;T&gt;</code>, appending
<code>?</code> to an <code>Option</code> expression:
<b>unwraps <code>Some</code></b> if present, or
<b>returns <code>None</code> early</b> from the enclosing function.
This propagates absence up the call stack without nested
<code>match</code> boilerplate.</p>
<p><code>?</code> works symmetrically for <code>Result&lt;T, E&gt;</code> —
covered in depth in the Errors section.</p>"##,
            code: r##"// Returns the first word of a string, or None if empty
fn first_word(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    // find(' ') returns Option<usize> — the byte index of the first space
    let end = trimmed.find(' ').unwrap_or(trimmed.len());
    Some(&trimmed[..end])
}

// ? propagates None early — no nested match needed
fn double_first_word_len(s: &str) -> Option<usize> {
    let word = first_word(s)?; // early-return None if first_word returns None
    Some(word.len() * 2)
}

fn main() {
    // match — most explicit
    match first_word("hello world") {
        Some(w) => println!("first word: '{w}'"),
        None    => println!("empty input"),
    }

    // if let — when you only care about Some
    if let Some(w) = first_word("  rust  ") {
        println!("trimmed first word: '{w}'");
    }

    // Option methods
    let maybe: Option<i32> = Some(42);
    let nothing: Option<i32> = None;

    println!("{}", maybe.unwrap_or(0));                       // 42
    println!("{}", nothing.unwrap_or(0));                     // 0
    println!("{:?}", maybe.map(|x| x * 2));                   // Some(84)
    println!("{:?}", maybe.filter(|&x| x > 100));             // None
    println!("{:?}", nothing.unwrap_or_else(|| 7 * 6));       // 42

    // and_then chains Option-returning functions (flatMap)
    let result = Some("  hello world  ")
        .map(str::trim)
        .and_then(|s| first_word(s));
    println!("chained: {result:?}");

    // ? propagation
    println!("{:?}", double_first_word_len("hello world")); // Some(10)
    println!("{:?}", double_first_word_len(""));            // None
    println!("{:?}", double_first_word_len("singleword"));  // Some(10)
}
"##,
            notes: vec![
                "`Option<T>` is just an enum in std — `Some(T)` or `None`. No null pointer, no `undefined`, no runtime surprise.",
                "`.unwrap()` panics on `None`. Use it only in tests and prototypes. Prefer `.unwrap_or()`, `.expect()`, or `?` in production code.",
                "`.map()` transforms the inner value; `.and_then()` flat-maps (your closure returns an `Option`). Together they let you chain nullable operations cleanly.",
                "`?` in an `Option`-returning function: unwraps `Some` or returns `None` early. Eliminate all nested `match` boilerplate.",
                "`if let Some(x) = opt { … }` is idiomatic for \"do something only when a value is present\".",
                "`s.len()` on a string slice is bytes. `\"hello\".len()` is 5, but `\"🦀\".len()` is 4 (UTF-8 bytes), not 1.",
            ],
        },

        // ── 6. early-return-patterns ──────────────────────────────────────────
        Lesson {
            id: "early-return-patterns",
            category: "Control Flow",
            title: "Early Returns & Guard Clauses",
            description: r##"<p>TypeScript developers often write guard clauses like
<code>if (x == null) return;</code> to exit early when a precondition fails,
keeping the "happy path" at the top level. Rust provides several ergonomic
constructs that achieve the same goal with full compiler support.</p>

<h3>let-else (Rust 1.65+)</h3>
<pre>let PATTERN = EXPR else {
    // must diverge: return / break / continue / panic!
};</pre>
<p>If the pattern does <em>not</em> match, the <code>else</code> block runs and
<b>must diverge</b> (it must <code>return</code>, <code>break</code>,
<code>continue</code>, or <code>panic!</code>). After the statement, the bound
variables are in scope in the enclosing block. This avoids the rightward
drift of nested <code>if let</code> chains — each guard stays at the same
indentation level.</p>

<h3>if let</h3>
<p><code>if let Some(x) = opt { … } else { … }</code> — use when you want to
conditionally run code based on a pattern match. Can be chained with
<code>else if let</code>.</p>

<h3>? operator (recap)</h3>
<p>In any function returning <code>Option&lt;T&gt;</code> or
<code>Result&lt;T, E&gt;</code>, <code>expr?</code>: unwraps the success
variant, or <b>returns early</b> with the failure variant. For
<code>Result</code> it also calls <code>From::from</code> on the error, so
different error types can be automatically converted — enabling a flat
sequence of <code>?</code>-terminated expressions instead of nested callbacks.</p>

<h3>Pattern</h3>
<p>Idiomatic Rust functions that can fail read as a flat sequence of
steps — each line either succeeds and gives you a value, or
<code>?</code>/<code>let-else</code> exits early. No deep nesting, no
callback pyramid, no try/catch blocks.</p>"##,
            code: r##"// let-else: unwrap or return early — no nesting
fn parse_positive(s: &str) -> Option<u32> {
    let Ok(n) = s.trim().parse::<u32>() else {
        return None; // parse failed — must diverge
    };
    if n == 0 {
        return None;
    }
    Some(n)
}

// Multiple guard clauses keep the happy path flat
fn first_two(v: &[i32]) -> Option<(i32, i32)> {
    let Some(&a) = v.first() else { return None; };
    let Some(&b) = v.get(1)  else { return None; };
    Some((a, b))
}

// ? in a Result context — the canonical use case
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.trim().parse()?; // ? converts and propagates on error
    Ok(n * 2)
}

// A realistic multi-step parsing function — reads as a flat pipeline
fn extract_port(addr: &str) -> Option<u16> {
    // e.g. "127.0.0.1:8080"
    let colon = addr.rfind(':')?;           // ? propagates None
    let port_str = addr.get(colon + 1..)?;  // ? propagates None
    port_str.parse().ok()                   // parse().ok() converts Result → Option
}

fn main() {
    // parse_positive
    for s in ["42", "-1", "0", "abc", "255"] {
        println!("parse_positive({s:>5}) = {:?}", parse_positive(s));
    }
    println!();

    // first_two
    println!("{:?}", first_two(&[10, 20, 30])); // Some((10, 20))
    println!("{:?}", first_two(&[10]));          // None
    println!("{:?}", first_two(&[]));            // None
    println!();

    // Result with ?
    println!("{:?}", parse_and_double("21")); // Ok(42)
    println!("{:?}", parse_and_double("xx")); // Err(...)
    println!();

    // extract_port
    println!("{:?}", extract_port("127.0.0.1:8080")); // Some(8080)
    println!("{:?}", extract_port("no-port-here"));    // None
    println!("{:?}", extract_port("host:99999"));      // None (> u16::MAX)
}
"##,
            notes: vec![
                "`let-else` (Rust 1.65+) is the clearest way to write guard clauses: unwrap a pattern or diverge. No nesting, bound variable stays in scope.",
                "The `else` block of `let-else` *must* diverge — `return`, `break`, `continue`, or `panic!()`. The compiler enforces this.",
                "`?` in `Option`-returning functions propagates `None`; in `Result`-returning functions it propagates `Err` (with `From` conversion).",
                "You can chain multiple `let-else` guards at the same indentation level — each line reads as \"ensure this or bail\".",
                "`result.ok()` converts `Result<T, E>` to `Option<T>`, discarding the error. Useful when you only care about success.",
                "Idiomatic error-handling code looks like a flat sequence of `?`-terminated steps — no try/catch nesting, no callback pyramid.",
            ],
        },
    ]
}
