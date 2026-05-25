use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "cargo",
            category: "Tooling & Packages",
            title: "Cargo: Build System & Package Manager",
            description: r##"<p><b>Cargo</b> is Rust's all-in-one build system, package manager, test runner,
and documentation generator — shipped with the language. Compare to <code>npm</code> +
<code>tsc</code> + <code>jest</code> combined, with zero configuration.</p>

<h3>Essential commands</h3>
<ul>
  <li><code>cargo new my-app</code> — create a binary crate (<code>src/main.rs</code>)</li>
  <li><code>cargo new my-lib --lib</code> — create a library crate (<code>src/lib.rs</code>)</li>
  <li><code>cargo check</code> — type-check without producing a binary (fastest feedback loop)</li>
  <li><code>cargo build</code> — compile in debug mode</li>
  <li><code>cargo run</code> — build + execute</li>
  <li><code>cargo test</code> — run all tests</li>
  <li><code>cargo add serde --features derive</code> — add a dependency (like <code>npm install</code>)</li>
  <li><code>cargo doc --open</code> — build and open documentation</li>
</ul>

<h3>Key files</h3>
<ul>
  <li><code>Cargo.toml</code> — manifest: name, version, edition, dependencies (like
      <code>package.json</code>)</li>
  <li><code>Cargo.lock</code> — exact resolved versions (like <code>package-lock.json</code>);
      commit for binaries, add to <code>.gitignore</code> for libraries</li>
</ul>

<h3>Profiles</h3>
<ul>
  <li><b>dev</b> (default) — fast compile, debug symbols, no LLVM optimizations</li>
  <li><b>release</b> (<code>--release</code>) — slow compile, full optimizations;
      runtime can be 10–100× faster. Always use for benchmarking.</li>
</ul>

<h3>Workspaces</h3>
<p>Multiple crates sharing one <code>Cargo.lock</code> and <code>target/</code> directory —
like npm workspaces. Add <code>[workspace] members = ["crate-a", "crate-b"]</code> to a
root <code>Cargo.toml</code>.</p>"##,
            code: r#"// Build and run this snippet with:
//   cargo new demo && cd demo && cargo run
//
// Cargo.toml structure (like package.json):
//
//   [package]
//   name    = "demo"
//   version = "0.1.0"
//   edition = "2021"
//
//   [dependencies]
//   serde = { version = "1", features = ["derive"] }
//
//   [profile.release]
//   opt-level = 3  # default for release
//
// Common workflow:
//   cargo check            # fast type-check (no binary) — use on every save
//   cargo build            # debug build → target/debug/demo
//   cargo build --release  # optimized → target/release/demo
//   cargo test             # run all #[test] functions
//   cargo clippy           # lint
//   cargo fmt              # format

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                (a, b) = (b, a + b);
            }
            b
        }
    }
}

fn main() {
    println!("Build me with `cargo run`");
    println!("For production: cargo build --release");
    println!();
    for i in [0, 1, 5, 10, 20] {
        println!("fib({:2}) = {}", i, fibonacci(i));
    }
}
"#,
            notes: vec![
                "`cargo check` is 3–5× faster than `cargo build` — make it your save-on-change command for the tightest feedback loop",
                "`crates.io` is the official registry; every crate's documentation is automatically published at `docs.rs`",
                "Debug builds include iterator overhead, bounds checks, and no inlining — NEVER benchmark a debug build",
                "Workspaces share a `target/` directory, so crate A's build artifacts are cached when building crate B",
            ],
        },
        Lesson {
            id: "rustfmt-clippy",
            category: "Tooling & Packages",
            title: "rustfmt and clippy",
            description: r##"<p>Rust ships two essential dev tools out of the box — no npm installs, no config files
needed.</p>

<h3>rustfmt — canonical formatting</h3>
<p><code>cargo fmt</code> applies the official Rust style: 4-space indentation, trailing
commas in multi-line constructs, consistent brace placement. Non-negotiable in every
major Rust project. Compare to Prettier for TypeScript — except it's built in and covers
the entire language. No debates about style; one format, always.</p>

<h3>Clippy — ~600 lints</h3>
<p><code>cargo clippy</code> catches bugs, anti-patterns, and non-idiomatic code.
Categories include:</p>
<ul>
  <li><b>correctness</b>: <code>absurd_extreme_comparisons</code>, <code>eq_op</code>
      (comparing a value to itself)</li>
  <li><b>performance</b>: <code>needless_clone</code>, <code>unnecessary_to_owned</code>,
      <code>vec_init_then_push</code></li>
  <li><b>style</b>: <code>needless_return</code>, <code>redundant_closure</code>,
      <code>map_unwrap_or</code></li>
  <li><b>pedantic</b> (opt-in): stricter set — add <code>#![warn(clippy::pedantic)]</code>
      to your crate root</li>
</ul>
<p>Compare to ESLint for TypeScript — except Clippy is maintained by the core team,
zero config out of the box, and many fixes can be auto-applied.</p>

<h3>In CI</h3>
<pre><code>cargo fmt --check          # fail if formatting differs
cargo clippy -- -D warnings # fail on any warning</code></pre>

<p>Auto-fix: <code>cargo clippy --fix</code> applies safe suggestions automatically.</p>"##,
            code: r#"// Run `cargo clippy` on this file — it will flag several anti-patterns.
// Each comment shows what Clippy says and what the idiomatic fix is.

fn add(a: i32, b: i32) -> i32 {
    return a + b; // clippy::needless_return → just write `a + b`
}

fn double(vals: &[i32]) -> Vec<i32> {
    // clippy::redundant_closure: `|x| x * 2` could be written differently,
    // but here it's idiomatic. Clippy is smart about when to flag this.
    vals.iter().map(|x| x * 2).collect()
}

fn clamp_positive(x: i32) -> i32 {
    // clippy::manual_clamp → use x.clamp(0, i32::MAX)
    if x < 0 { 0 } else { x }
}

#[allow(clippy::needless_collect)] // suppress one specific lint
fn count_even(nums: &[i32]) -> usize {
    let evens: Vec<_> = nums.iter().filter(|&&n| n % 2 == 0).collect();
    evens.len()
    // clippy would normally suggest: nums.iter().filter(|&&n| n % 2 == 0).count()
}

fn main() {
    println!("add(3, 4)       = {}", add(3, 4));
    println!("double([1,2,3]) = {:?}", double(&[1, 2, 3]));
    println!("clamp(-5)       = {}", clamp_positive(-5));
    println!("count_even      = {}", count_even(&[1, 2, 3, 4, 5, 6]));

    // cargo fmt enforces:
    //   - 4-space indentation (never tabs)
    //   - trailing commas in multi-line match/struct/vec
    //   - import grouping: std, external crates, local
}
"#,
            notes: vec![
                "`cargo fmt` is idempotent and purely cosmetic — safe to run automatically on every file save via the editor's format-on-save setting",
                "`cargo clippy -- -D warnings` is the standard CI incantation: any warning becomes a build failure",
                "`#[allow(clippy::lint_name)]` silences one lint for one item; `#![allow(...)]` at crate root silences globally (use sparingly)",
                "`cargo clippy --fix` auto-applies safe suggestions — good to run once after adding `#![warn(clippy::pedantic)]`",
            ],
        },
        Lesson {
            id: "modules",
            category: "Tooling & Packages",
            title: "Modules and Visibility",
            description: r##"<p>Rust's module system organizes code and controls what's public API vs internal
implementation detail.</p>

<h3>Declaring modules</h3>
<ul>
  <li><code>mod foo { ... }</code> — inline module, defined right here</li>
  <li><code>mod foo;</code> — loads from <code>foo.rs</code> or <code>foo/mod.rs</code>
      on disk</li>
</ul>

<h3>Visibility modifiers</h3>
<ul>
  <li>(default) private — accessible only within the current module and its descendants</li>
  <li><code>pub</code> — accessible to anyone</li>
  <li><code>pub(crate)</code> — visible anywhere in the crate, not to external consumers</li>
  <li><code>pub(super)</code> — visible to the parent module only</li>
  <li><code>pub(in path)</code> — visible to a specific ancestor module</li>
</ul>

<h3>Imports and re-exports</h3>
<p><code>use std::collections::HashMap;</code> — like ES6 <code>import</code>.<br>
<code>pub use inner::Foo;</code> — re-export; callers see <code>Foo</code> at this
module level (great for flattening a public API).</p>

<h3>Typical file layout</h3>
<pre><code>src/
  main.rs          ← declares top-level modules with `mod`
  auth/
    mod.rs         ← auth module root
    jwt.rs         ← auth::jwt submodule
  db.rs            ← db module</code></pre>

<p>Compare to Node.js: a Rust module is like a JS file, but you must explicitly
<em>declare</em> it with <code>mod</code> — the compiler does not auto-discover files.</p>"##,
            code: r#"mod math {
    // Private — only accessible inside `math` and its children
    fn square(x: i32) -> i32 {
        x * x
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sum_of_squares(a: i32, b: i32) -> i32 {
        square(a) + square(b) // can call private `square` from within math
    }

    // Nested submodule
    pub mod trig {
        pub fn approx_sin(x: f64) -> f64 {
            // Taylor series: sin(x) ≈ x - x³/6 (accurate for small x)
            x - (x * x * x) / 6.0
        }

        pub(super) fn internal_helper() -> &'static str {
            "only math can see me" // pub(super) = visible to `math`, not root
        }
    }
}

// Bring specific items into scope (like ES6 named imports)
use math::add;
use math::trig::approx_sin;

fn main() {
    println!("2 + 3          = {}", add(2, 3));
    println!("sum_of_squares = {}", math::sum_of_squares(3, 4)); // 9 + 16 = 25
    println!("sin(0.1) ≈     {:.6}", approx_sin(0.1));

    // math::square(5) → ERROR: `square` is private
    // math::trig::internal_helper() → ERROR: pub(super) restricts to `math`
}
"#,
            notes: vec![
                "`mod` declares a module; `use` imports names from it — these are separate steps, unlike Node.js `require`/`import` which do both",
                "The compiler looks for `mod foo;` as `src/foo.rs` OR `src/foo/mod.rs` — the latter is preferred for modules with submodules",
                "`pub use inner::Type` re-exports `Type` at the current module level — use it to present a clean public API while keeping the internal structure flexible",
                "`crate::` is an absolute path from the crate root; `super::` goes up one module level — prefer absolute paths for clarity",
            ],
        },
        Lesson {
            id: "tests",
            category: "Tooling & Packages",
            title: "Writing Tests",
            description: r##"<p>Rust's test framework is built in — no Jest, no Mocha, no config files. Run all
tests with <code>cargo test</code>.</p>

<h3>Unit tests</h3>
<p>Annotate functions with <code>#[test]</code> inside a
<code>#[cfg(test)] mod tests</code> block in the same file as the code under test.
<code>#[cfg(test)]</code> excludes the block from production builds — zero binary size
overhead. Tests inside the module can access private items.</p>

<h3>Assertions</h3>
<ul>
  <li><code>assert!(expr)</code> — panics with a message if false</li>
  <li><code>assert_eq!(left, right)</code> — panics with a diff if unequal
      (requires <code>PartialEq + Debug</code>)</li>
  <li><code>assert_ne!(left, right)</code> — panics if equal</li>
  <li><code>#[should_panic]</code> — test passes only if the body panics</li>
  <li><code>#[should_panic(expected = "msg")]</code> — also checks the panic message</li>
</ul>

<h3>Integration tests</h3>
<p>Files in <code>tests/</code> at the crate root are compiled as separate crates with
access only to the public API — great for end-to-end function tests.</p>

<h3>Filtering and options</h3>
<pre><code>cargo test              # run everything
cargo test add          # run tests whose name contains "add"
cargo test -- --nocapture   # print stdout during tests
cargo test -- --test-threads=1  # run serially (for shared state)</code></pre>"##,
            code: r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("division by zero");
    }
    a / b
}

// Private helper — visible to tests in this file via `use super::*`
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // In a real project run: cargo test
    // The #[test] functions below don't run via main().
    println!("add(2, 3)          = {}", add(2, 3));
    println!("divide(10.0, 4.0)  = {}", divide(10.0, 4.0));
    println!("is_even(4)         = {}", is_even(4));
}

#[cfg(test)] // entire block excluded from release binary
mod tests {
    use super::*; // access private items from the parent module

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
        assert_ne!(add(2, 2), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
        assert!(add(-5, -3) < 0);
    }

    #[test]
    fn test_divide() {
        let result = divide(10.0, 4.0);
        assert!((result - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_divide_by_zero_panics() {
        divide(1.0, 0.0); // must panic; test passes if it does
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
    }
}
"#,
            notes: vec![
                "`#[cfg(test)]` ensures the test module is compiled only during `cargo test` — the production binary is not affected",
                "Tests can return `Result<(), E>` and use `?` for propagation — a returned `Err` counts as a test failure with a clean error message",
                "`cargo test` runs tests in parallel by default — ensure tests don't mutate shared global state, or use `--test-threads=1`",
                "Integration tests in `tests/` only have access to the public API — they help catch accidental breaking changes",
            ],
        },
        Lesson {
            id: "benchmarks-and-profiling",
            category: "Tooling & Packages",
            title: "Benchmarks and Profiling",
            description: r##"<p>Measuring performance correctly is essential before optimizing.</p>

<h3>Stable Rust: criterion (community standard)</h3>
<p>The <code>criterion</code> crate applies statistical rigor: warm-up runs, outlier
detection, and HTML reports. Add it as a dev-dependency:</p>
<pre><code>[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name    = "my_bench"
harness = false</code></pre>
<p>Run with <code>cargo bench</code>. Results appear in <code>target/criterion/</code>.</p>

<h3>Nightly: built-in #[bench]</h3>
<p><code>#[bench]</code> attribute is nightly-only and less statistically robust.
Prefer <code>criterion</code> on stable.</p>

<h3>Profiling tools</h3>
<ul>
  <li><b>Always</b> profile <code>--release</code> builds: debug builds run 10–100×
      slower and have different hot paths</li>
  <li><code>cargo flamegraph</code> — generates a flamegraph SVG (requires
      <code>perf</code> on Linux, <code>dtrace</code> on macOS)</li>
  <li><code>samply</code> — macOS-friendly profiler with Firefox Profiler UI</li>
  <li><code>Instruments</code> — Xcode profiler; works well on macOS</li>
  <li><code>cargo bloat</code> — shows which functions contribute most to binary size</li>
</ul>

<h3>std::hint::black_box</h3>
<p>Use <code>std::hint::black_box(val)</code> in benchmarks to prevent LLVM from
optimizing away computations with no observable side-effects.</p>"##,
            code: r#"use std::time::Instant;
use std::hint::black_box;

fn naive_sum(n: u64) -> u64 {
    (1..=n).sum()
}

fn formula_sum(n: u64) -> u64 {
    // Gauss: n*(n+1)/2
    n * (n + 1) / 2
}

/// A minimal hand-rolled timing helper — use `criterion` for real benchmarks.
fn time_it<F, R>(label: &str, mut f: F) -> R
where
    F: FnMut() -> R,
{
    let start = Instant::now();
    let result = f();
    println!("{:<20} {:>10?}", label, start.elapsed());
    result
}

fn main() {
    let n = 10_000_000u64;

    // black_box prevents the optimizer from pre-computing the constant result
    let r1 = time_it("naive_sum", || naive_sum(black_box(n)));
    let r2 = time_it("formula_sum", || formula_sum(black_box(n)));

    assert_eq!(r1, r2, "results must agree");
    println!("\nBoth = {}", r1);
    println!("\nNote: run with `cargo run --release` for meaningful numbers.");
    println!("Debug builds have bounds checks, no inlining, and iterator overhead.");
}
"#,
            notes: vec![
                "`Instant` is monotonic — safe for measuring elapsed time; `SystemTime` can go backwards due to NTP adjustments",
                "Always profile `--release` builds: debug builds run 10–100× slower and profile completely different code paths",
                "`std::hint::black_box(val)` is a no-op at runtime but tells LLVM \"treat this value as observable\" — prevents dead-code elimination in micro-benchmarks",
                "`cargo flamegraph` is the fastest path to finding hot functions; install with `cargo install flamegraph` then run `cargo flamegraph --bin my-app`",
            ],
        },
    ]
}
