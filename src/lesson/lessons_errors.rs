use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "result-and-questionmark",
            category: "Errors",
            title: "Result<T, E> and the ? Operator",
            description: r#"<p>
  Rust models recoverable errors as <b>values</b>, not exceptions.
  Every function that can fail returns a <code>Result&lt;T, E&gt;</code> —
  an enum with two variants:
</p>
<ul>
  <li><code>Ok(T)</code> — success, carrying the value</li>
  <li><code>Err(E)</code> — failure, carrying the error</li>
</ul>
<p>
  This is similar to Go's <code>(value, error)</code> return idiom,
  but enforced by the type system: you <b>cannot</b> silently ignore a <code>Result</code>
  (the compiler warns on unused results).
</p>

<h3>The ? operator</h3>
<p>
  <code>?</code> placed after a <code>Result</code> expression does two things:
</p>
<ul>
  <li>On <code>Ok(v)</code> — unwraps to <code>v</code> and continues.</li>
  <li>On <code>Err(e)</code> — calls <code>From::from(e)</code> to coerce the error type,
      then <b>early-returns</b> <code>Err(...)</code> from the enclosing function.</li>
</ul>
<p>
  The <code>From</code> conversion means you can often mix error types in one function
  as long as each can convert into the function's declared error type.
</p>

<h3>Box&lt;dyn Error&gt;</h3>
<p>
  For quick scripts or examples, use <code>Box&lt;dyn std::error::Error&gt;</code>
  as the error type. Any type that implements <code>std::error::Error</code>
  converts into it automatically, so <code>?</code> works across error types.
</p>"#,
            code: r#"use std::num::ParseIntError;

// Simple: parse a string to i32 and double it
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?; // ? early-returns Err on failure
    Ok(n * 2)
}

// Box<dyn Error>: accepts any error type — ? uses From for coercion
fn sum_two(a: &str, b: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let x: i32 = a.trim().parse()?; // ParseIntError coerces into Box<dyn Error>
    let y: i32 = b.trim().parse()?;
    Ok(x + y)
}

fn main() {
    // The happy path
    println!("{:?}", parse_and_double("21")); // Ok(42)
    println!("{:?}", sum_two("10", "32"));    // Ok(42)

    // Error paths — the ? operator propagates these up
    println!("{:?}", parse_and_double("x"));  // Err(invalid digit found in string)
    println!("{:?}", sum_two("10", "y"));     // Err(invalid digit found in string)

    // Pattern matching on Result
    match parse_and_double("5") {
        Ok(n)  => println!("doubled: {}", n),
        Err(e) => println!("failed: {}", e),
    }

    // if let for the success case only
    if let Ok(n) = parse_and_double("7") {
        println!("got: {}", n);
    }
}"#,
            notes: vec![
                "Result<T, E> is just an enum — Ok(T) or Err(E). The type system forces you to handle both arms.",
                "? is syntactic sugar for: match result { Ok(v) => v, Err(e) => return Err(From::from(e)) }.",
                "The From conversion in ? allows mixing error types as long as each implements From<PriorError>.",
                "Box<dyn std::error::Error> is the easy catch-all; prefer a typed error enum in library code.",
                "Unlike Go, you cannot accidentally shadow an error by forgetting to check — the compiler warns on unused Results.",
            ],
        },
        Lesson {
            id: "panic-vs-result",
            category: "Errors",
            title: "panic! vs Result",
            description: r#"<p>
  Rust distinguishes between two fundamentally different failure modes:
</p>
<ul>
  <li><b>Recoverable errors</b> — bad input, missing file, network timeout.
      Model with <code>Result&lt;T, E&gt;</code> and let the caller decide how to handle it.</li>
  <li><b>Unrecoverable bugs</b> — violated invariants, logic errors that should never happen in correct code.
      Use <code>panic!</code>, which unwinds the stack (or aborts, if configured).</li>
</ul>

<h3>panic! family</h3>
<ul>
  <li><code>panic!("msg")</code> — explicit panic with a message</li>
  <li><code>unwrap()</code> — panics on <code>Err</code> or <code>None</code> with a generic message</li>
  <li><code>expect("msg")</code> — panics with your message; prefer over <code>unwrap()</code></li>
  <li><code>assert!(cond)</code> / <code>assert_eq!(a, b)</code> — panic if condition fails</li>
  <li><code>unreachable!()</code> — marks a code path that must never execute</li>
  <li><code>todo!()</code> — placeholder for unimplemented code; panics if reached</li>
</ul>

<h3>When is unwrap/expect acceptable?</h3>
<ul>
  <li>In tests and examples where panicking is the right failure mode</li>
  <li>When you have <b>proven</b> by logic that the value is <code>Ok</code>/<code>Some</code>
      and adding error handling would obscure the invariant</li>
  <li>In <code>main()</code> for quick scripts where propagating with <code>?</code> isn't set up</li>
</ul>

<h3>catch_unwind</h3>
<p>
  <code>std::panic::catch_unwind</code> can catch panics, but it's rarely the right tool.
  It's mainly used by test frameworks and FFI boundary code.
</p>"#,
            code: r#"fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string()) // recoverable — caller decides
    } else {
        Ok(a / b)
    }
}

// Panics are for violated invariants — the caller guarantees a non-empty slice
fn first(v: &[i32]) -> i32 {
    assert!(!v.is_empty(), "caller contract: slice must not be empty");
    v[0]
}

fn should_never_happen(x: u8) -> &'static str {
    match x {
        1 => "one",
        2 => "two",
        _ => unreachable!("only 1 and 2 are valid inputs by construction"),
    }
}

fn _not_implemented_yet() -> i32 {
    todo!("implement this in the next sprint")
}

fn main() {
    // Use Result for expected failure paths
    match divide(10.0, 2.0) {
        Ok(v)  => println!("result: {}", v),   // 5.0
        Err(e) => println!("error: {}", e),
    }
    println!("{:?}", divide(5.0, 0.0)); // Err("division by zero")

    // expect: only when you know it can't fail — state the invariant in the message
    let n: i32 = "42".parse().expect("hardcoded literal is always a valid i32");
    println!("parsed: {}", n);

    println!("first: {}", first(&[10, 20, 30]));
    println!("label: {}", should_never_happen(1));

    // first(&[]); // would panic — violated caller contract
}"#,
            notes: vec![
                "panic! is for bugs; Result is for expected failures. Mixing them up makes code hard to reason about.",
                "Prefer expect(\"invariant description\") over unwrap() — the message appears in the panic and aids debugging.",
                "In library code, never panic on user input — always return Result or Option so callers can handle it.",
                "unreachable!() and todo!() both panic at runtime; they document intent clearly in the source.",
                "catch_unwind is rarely needed outside test harnesses and FFI boundary code.",
            ],
        },
        Lesson {
            id: "custom-error-types",
            category: "Errors",
            title: "Custom Error Types",
            description: r#"<p>
  For library code, define a dedicated error enum so callers can pattern-match on specific variants.
  Three traits are typically implemented:
</p>
<ul>
  <li><code>std::fmt::Display</code> — human-readable message for end users</li>
  <li><code>std::error::Error</code> — standard trait; enables <code>source()</code> chain and
      <code>Box&lt;dyn Error&gt;</code> compatibility</li>
  <li><code>From&lt;PriorError&gt;</code> — lets <code>?</code> convert automatically</li>
</ul>

<h3>Typical shape</h3>
<pre><code>#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(String),
}
impl std::fmt::Display for AppError { ... }
impl std::error::Error for AppError {
    fn source(&amp;self) -> Option&lt;&amp;(dyn Error + 'static)&gt; { ... }
}
impl From&lt;std::io::Error&gt; for AppError { ... }
</code></pre>

<h3>Popular crates (don't use in this app — describe only)</h3>
<ul>
  <li><b>thiserror</b> — for library crates; derives <code>Display</code>, <code>Error</code>,
      and <code>From</code> with a simple macro. Zero overhead.</li>
  <li><b>anyhow</b> — for application crates; provides a single <code>anyhow::Error</code>
      type that wraps any error with a backtrace. Great for binaries.</li>
</ul>"#,
            code: r#"use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum LoadError {
    Parse(ParseIntError),           // wraps an existing error
    InvalidRange(String),           // carries a message
}

// 1. Human-readable display — what an end user (or log) sees
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Parse(e)           => write!(f, "parse error: {}", e),
            LoadError::InvalidRange(msg)  => write!(f, "invalid range: {}", msg),
        }
    }
}

// 2. Satisfy std::error::Error — source() links the error chain
impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Parse(e)           => Some(e),
            LoadError::InvalidRange(_)    => None,
        }
    }
}

// 3. From impl: lets ? auto-convert ParseIntError → LoadError
impl From<ParseIntError> for LoadError {
    fn from(e: ParseIntError) -> Self {
        LoadError::Parse(e)
    }
}

fn load_percentage(s: &str) -> Result<u8, LoadError> {
    let n: i32 = s.trim().parse()?; // ? calls From<ParseIntError> for us
    if !(0..=100).contains(&n) {
        return Err(LoadError::InvalidRange(
            format!("{} is not in 0..=100", n),
        ));
    }
    Ok(n as u8)
}

fn main() {
    println!("{:?}", load_percentage("42"));   // Ok(42)
    println!("{:?}", load_percentage("bad"));  // Err(Parse(...))
    println!("{:?}", load_percentage("150"));  // Err(InvalidRange(...))

    // Display the human-readable message
    if let Err(e) = load_percentage("bad") {
        println!("error: {}", e); // "parse error: invalid digit found in string"
    }

    // Callers can branch on specific variants
    match load_percentage("200") {
        Ok(n)                          => println!("loaded: {}", n),
        Err(LoadError::Parse(e))       => println!("couldn't parse: {}", e),
        Err(LoadError::InvalidRange(m))=> println!("out of range: {}", m),
    }
}"#,
            notes: vec![
                "Implement Display + Error + From on your error enum — that's the full contract for interoperating with ?.",
                "source() in std::error::Error enables error chain inspection (e.cause() in older APIs).",
                "From<OtherError> is what ? uses for the coercion step — without it, ? won't compile across error types.",
                "thiserror generates Display + Error + From via macros; anyhow provides a Box-like catch-all for binaries.",
                "Libraries should expose typed error enums so callers can match on variants; binaries can use anyhow freely.",
            ],
        },
        Lesson {
            id: "error-handling-patterns",
            category: "Errors",
            title: "Error Handling Patterns",
            description: r#"<p>
  Beyond <code>?</code> and <code>match</code>, <code>Result</code> ships with a rich set of
  combinators for transforming and chaining fallible operations without nested matches.
</p>

<h3>Key combinators</h3>
<ul>
  <li><code>map(|v| ...)</code> — transform the <code>Ok</code> value, pass <code>Err</code> through</li>
  <li><code>map_err(|e| ...)</code> — transform the <code>Err</code> value, pass <code>Ok</code> through</li>
  <li><code>and_then(|v| ...)</code> — chain a fallible operation; short-circuits on <code>Err</code></li>
  <li><code>or_else(|e| ...)</code> — try a fallback on failure</li>
  <li><code>unwrap_or(default)</code> — extract value or use a default</li>
  <li><code>unwrap_or_else(|| ...)</code> — extract value or compute a fallback lazily</li>
</ul>

<h3>The newtype vs Box vs anyhow trade-off</h3>
<ul>
  <li><b>Typed enum</b> (recommended for libraries) — callers can match on variants</li>
  <li><code>Box&lt;dyn std::error::Error&gt;</code> — easy, no dependencies; erases the type</li>
  <li><b>anyhow::Error</b> (binaries only) — best DX; includes backtrace; not matchable by callers</li>
</ul>

<h3>Collecting Results</h3>
<p>
  <code>Iterator::collect</code> can fold an iterator of <code>Result&lt;T, E&gt;</code>
  into a single <code>Result&lt;Vec&lt;T&gt;, E&gt;</code> — failing fast on the first error.
</p>

<h3>vs JavaScript try/catch</h3>
<p>
  In JS/TS, <code>throw</code> can happen anywhere and is invisible at the call site.
  Rust forces you to acknowledge every fallible function: either <code>?</code> it,
  <code>unwrap()</code> it, or <code>match</code> it.
  Error paths are <b>visible and impossible to silently ignore</b>.
</p>"#,
            code: r#"use std::num::ParseIntError;

// map: transform the Ok value, pass Err through unchanged
fn doubled(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>().map(|n| n * 2)
}

// map_err + and_then: reshape the error type, then chain a validation step
fn parse_positive(s: &str) -> Result<u32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|e| format!("parse failed: {}", e))
        .and_then(|n| {
            if n >= 0 {
                Ok(n as u32)
            } else {
                Err(format!("{} is negative", n))
            }
        })
}

fn main() {
    // map: Ok(21) → Ok(42)
    println!("{:?}", doubled("21")); // Ok(42)
    println!("{:?}", doubled("x"));  // Err(invalid digit found in string)

    // and_then chaining
    println!("{:?}", parse_positive("42")); // Ok(42)
    println!("{:?}", parse_positive("-1")); // Err("-1 is negative")
    println!("{:?}", parse_positive("x"));  // Err("parse failed: ...")

    // unwrap_or / unwrap_or_else: provide fallbacks without panicking
    let n: i32 = "bad".parse().unwrap_or(0);
    println!("fallback: {}", n); // 0

    let n2: i32 = "bad".parse().unwrap_or_else(|_| -1);
    println!("lazy fallback: {}", n2); // -1

    // or_else: try another Result on failure
    let result: Result<i32, _> = "bad"
        .parse::<i32>()
        .or_else(|_| "42".parse::<i32>());
    println!("{:?}", result); // Ok(42)

    // Collect: Vec<Result<T,E>> → Result<Vec<T>, E> (fails fast on first Err)
    let strings = vec!["1", "2", "3"];
    let nums: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("{:?}", nums); // Ok([1, 2, 3])

    let mixed = vec!["1", "oops", "3"];
    let bad: Result<Vec<i32>, _> = mixed.iter().map(|s| s.parse::<i32>()).collect();
    println!("{:?}", bad); // Err(invalid digit found in string)
}"#,
            notes: vec![
                "and_then is the monadic bind for Result — use it to chain fallible steps without nesting.",
                "map_err is essential when composing functions that return different error types before using ?.",
                "unwrap_or_else is lazy — the closure runs only on Err, unlike unwrap_or which always evaluates the default.",
                "Collecting an iterator of Results into Result<Vec<T>, E> is idiomatic and fails fast on the first error.",
                "Prefer combinators over nested match for short pipelines; switch to ? and explicit match for complex logic.",
            ],
        },
    ]
}
