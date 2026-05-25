use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "io",
            category: "Standard Library",
            title: "Files and io::Read/Write",
            description: r##"<p>Rust's I/O is built around the <code>Read</code> and <code>Write</code> traits
in <code>std::io</code> — similar to Node.js streams but synchronous and type-safe by
default.</p>

<h3>Common patterns</h3>
<ul>
  <li><code>std::fs::read_to_string("path")</code> — reads an entire file into a
      <code>String</code>; idiomatic for small files</li>
  <li><code>fs::File::open(path)?</code> + <code>BufReader::new(file)</code> — streaming
      reads; iterate lines with <code>.lines()</code></li>
  <li><code>fs::write("path", bytes)</code> — write bytes atomically</li>
  <li><code>BufWriter::new(file)</code> — batches small writes into larger syscalls;
      always wrap file writers in <code>BufWriter</code></li>
</ul>

<h3>The io::Write trait</h3>
<p><code>write!</code> and <code>writeln!</code> work on <em>anything</em> implementing
<code>Write</code>: files, sockets, <code>Vec&lt;u8&gt;</code>, TCP streams, custom
types. This is how Rust achieves I/O abstraction without class hierarchies.</p>
<p><code>io::Cursor&lt;Vec&lt;u8&gt;&gt;</code> implements both <code>Read</code> and
<code>Write</code> — great for testing I/O-heavy code without touching the filesystem.</p>

<h3>File I/O in real projects</h3>
<pre><code>use std::{fs, io::{self, BufRead}};

let text = fs::read_to_string("data.txt")?;          // whole file

let file = fs::File::open("data.txt")?;
for line in io::BufReader::new(file).lines() {       // streaming
    println!("{}", line?);
}</code></pre>"##,
            code: r#"use std::io::Write; // needed to call writeln! on arbitrary io::Write types

fn main() {
    // Vec<u8> implements io::Write — acts like an in-memory file.
    // Swap `Vec<u8>` for `BufWriter<File>` and the rest is identical.
    let mut buffer: Vec<u8> = Vec::new();

    writeln!(buffer, "Line 1: Hello from the Write trait").unwrap();
    writeln!(buffer, "Line 2: value = {}", 42).unwrap();
    writeln!(buffer, "Line 3: pi ≈ {:.4}", std::f64::consts::PI).unwrap();
    writeln!(buffer, "Line 4: hex  = {:#010x}", 255u32).unwrap();

    // Vec<u8> → String → print
    let content = String::from_utf8(buffer).expect("we only wrote valid UTF-8");
    print!("{}", content);

    // Parse back using standard string methods
    println!("Line count:  {}", content.lines().count());
    println!("First line:  {:?}", content.lines().next().unwrap_or(""));

    // io::Cursor<&[u8]> implements Read — useful for testing code that
    // reads from an io::Read without touching the filesystem:
    //   let mut cursor = std::io::Cursor::new(b"hello\nworld");
    //   let mut s = String::new();
    //   cursor.read_to_string(&mut s).unwrap();

    // Demonstrate that stdout itself is io::Write
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "Writing directly to locked stdout handle.").unwrap();
}
"#,
            notes: vec![
                "`BufReader` reduces the number of `read` syscalls when reading line-by-line — always wrap unbuffered readers for line iteration",
                "`BufWriter` batches small `write` calls — always wrap file writers; forgetting to flush can silently lose the last bytes",
                "`io::Cursor<Vec<u8>>` implements both `Read` and `Write` — ideal for testing I/O code without touching the filesystem",
                "The `?` operator propagates `io::Error` cleanly; prefer it over `.unwrap()` in any function that returns `io::Result<T>`",
            ],
        },
        Lesson {
            id: "string-formatting",
            category: "Standard Library",
            title: "Strings and Formatting",
            description: r##"<p>Rust's format macro system is powerful, type-safe, and checked at compile time.
Compare to JS template literals — but a type mismatch or wrong argument count is a
compile error, not a runtime surprise.</p>

<h3>Macros</h3>
<ul>
  <li><code>format!(...)</code> → allocates and returns a <code>String</code></li>
  <li><code>print!</code> / <code>println!</code> → stdout (with/without newline)</li>
  <li><code>eprint!</code> / <code>eprintln!</code> → stderr</li>
  <li><code>write!(w, ...)</code> / <code>writeln!</code> → any <code>io::Write</code></li>
</ul>

<h3>Format specifiers</h3>
<ul>
  <li><code>{}</code> — <code>Display</code> trait (human-readable)</li>
  <li><code>{:?}</code> — <code>Debug</code> trait (derive with <code>#[derive(Debug)]</code>)</li>
  <li><code>{:#?}</code> — pretty-printed Debug</li>
  <li><code>{:5}</code> — minimum width 5, left-aligned</li>
  <li><code>{:&gt;5}</code> — right-align in width 5</li>
  <li><code>{:0&gt;5}</code> — zero-pad on the left to width 5</li>
  <li><code>{:.2}</code> — 2 decimal places</li>
  <li><code>{:08.3}</code> — width 8, zero-padded, 3 decimal places</li>
  <li><code>{:x}</code> / <code>{:X}</code> — lowercase/uppercase hex</li>
  <li><code>{:b}</code> — binary; <code>{:o}</code> — octal</li>
</ul>

<h3>Named arguments (since Rust 1.58)</h3>
<pre><code>let name = "Alice";
println!("{name}");               // variable captured by name
println!("{0} and {0}", "echo"); // positional index
println!("{n}", n = 42);         // explicit named</code></pre>"##,
            code: r#"fn main() {
    // Basic captures
    let name  = "Alice";
    let score = 98.567_f64;
    println!("Player: {name}, Score: {score:.1}");

    // Alignment table — great for CLI output
    println!("\n{:<12} {:>6} {:>9}", "Item",      "Qty",  "Price");
    println!("{:-<12} {:->6} {:->9}", "",          "",     "");
    println!("{:<12} {:>6} {:>9.2}", "Apple",       12,    0.99_f64);
    println!("{:<12} {:>6} {:>9.2}", "Blueberry",    3,    4.50_f64);
    println!("{:<12} {:>6} {:>9.2}", "Cherry",      200,   0.05_f64);

    // Padding and number bases
    println!("\nzero-padded:  {:0>5}",   42);          // 00042
    println!("float:        {:08.3}",   3.14159_f64); // 0003.142
    println!("hex (lower):  {:x}",      255u32);      // ff
    println!("hex (#):      {:#010x}",  255u32);      // 0x000000ff
    println!("binary:       {:08b}",    42u8);         // 00101010

    // Debug vs Display
    let v = vec![1, 2, 3];
    println!("\nDebug:        {:?}",  v);
    println!("Pretty debug:\n{:#?}", v);

    // Named arg (explicit)
    let msg = format!("{greeting}, {name}!", greeting = "Hello", name = name);
    println!("\n{}", msg);
}
"#,
            notes: vec![
                "Implement `std::fmt::Display` on your types to enable `{}`; `Debug` is almost always `#[derive(Debug)]`",
                "Format strings are verified at compile time — wrong argument count or missing `Display` impl is a compile error, not a runtime panic",
                "For hot paths, write directly with `write!` to an existing `String` or `Vec<u8>` buffer to avoid repeated allocations",
                "`{:#?}` pretty-prints with indentation — invaluable for inspecting nested structs during debugging",
            ],
        },
        Lesson {
            id: "collections-deep-dive",
            category: "Standard Library",
            title: "Deeper into Collections",
            description: r##"<p>Beyond <code>Vec</code> and <code>HashMap</code>, the standard library has
specialized collections for performance-critical patterns:</p>

<h3>VecDeque — double-ended queue</h3>
<p>O(1) push/pop at <em>both</em> ends (Vec only does O(1) at the back). Backed by a
ring buffer. Perfect for BFS, sliding windows, and work-stealing.
<code>use std::collections::VecDeque;</code></p>

<h3>BinaryHeap — priority queue</h3>
<p>Max-heap by default. Wrap with <code>std::cmp::Reverse</code> for min-heap behavior.
O(log n) push/pop, O(1) peek. Perfect for Dijkstra, top-k, scheduling.
<code>use std::collections::BinaryHeap;</code></p>

<h3>HashSet / BTreeSet</h3>
<ul>
  <li><code>HashSet&lt;T&gt;</code> — O(1) insert/contains; unordered</li>
  <li><code>BTreeSet&lt;T&gt;</code> — O(log n); sorted; supports
      <code>.range()</code> for range queries</li>
</ul>

<h3>BTreeMap</h3>
<p>Like <code>HashMap</code> but keys are sorted. Use when you need ordered iteration
or range lookups (<code>.range(from..=to)</code>).</p>

<h3>LinkedList — rarely useful</h3>
<p>Cache-unfriendly; Rust's ownership rules make it awkward to traverse mutably.
Almost always prefer <code>VecDeque</code>.</p>"##,
            code: r#"use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::cmp::Reverse;

// BFS using VecDeque as the frontier queue
fn bfs(start: u32, edges: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new(); // (node, depth)
    let mut visited: HashSet<u32> = HashSet::new();
    let mut result = vec![];

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((node, depth)) = queue.pop_front() {
        result.push((node, depth));
        for &(a, b) in edges {
            let neighbor = if a == node { b } else if b == node { a } else { continue };
            if visited.insert(neighbor) {          // insert returns true if new
                queue.push_back((neighbor, depth + 1));
            }
        }
    }
    result
}

// Top-k largest values using a min-heap of size k
fn top_k(nums: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
    for &n in nums {
        heap.push(Reverse(n));
        if heap.len() > k {
            heap.pop(); // evict the current minimum
        }
    }
    let mut result: Vec<i32> = heap.into_iter().map(|Reverse(v)| v).collect();
    result.sort_unstable_by(|a, b| b.cmp(a)); // descending
    result
}

fn main() {
    let edges = [(1, 2), (1, 3), (2, 4), (3, 4), (4, 5)];
    println!("BFS from node 1:");
    for (node, depth) in bfs(1, &edges) {
        println!("  node {} at depth {}", node, depth);
    }

    let nums = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("\nTop 3 from {:?}:", nums);
    println!("  {:?}", top_k(&nums, 3));
}
"#,
            notes: vec![
                "`VecDeque` is a ring buffer — `push_back`/`pop_front` are O(1); converting to/from `Vec` is O(n) but `Vec::from(deque)` is efficient when the buffer is contiguous",
                "`BinaryHeap::peek()` is O(1) — call it before `pop()` when you want to inspect without consuming",
                "Wrap values in `Reverse<T>` for a min-heap; or negate integers (`-n`) as a quick trick for integer-only heaps",
                "`BTreeMap`/`BTreeSet` have predictable O(log n) performance and support `.range()` — prefer them when ordered iteration or range queries are needed",
            ],
        },
        Lesson {
            id: "time-and-instant",
            category: "Standard Library",
            title: "Time: Duration and Instant",
            description: r##"<p>Rust's time types live in <code>std::time</code>.</p>

<h3>Instant — monotonic clock (use for measurement)</h3>
<p><code>Instant::now()</code> captures a point in time. <code>start.elapsed()</code>
returns a <code>Duration</code>. <b>Monotonic</b> means it never goes backwards — safe
for timing code, implementing timeouts, and rate limiting.</p>

<h3>Duration — a span of time</h3>
<p>Constructors: <code>Duration::from_secs(2)</code>, <code>from_millis(500)</code>,
<code>from_nanos(100)</code>, <code>from_secs_f64(1.5)</code>.<br>
Accessors: <code>.as_millis()</code>, <code>.as_micros()</code>, <code>.as_nanos()</code>,
<code>.as_secs_f64()</code>.<br>
Arithmetic: <code>d1 + d2</code>, <code>d1 * 3</code>, <code>d1.checked_sub(d2)</code>.</p>

<h3>SystemTime — wall clock (use for timestamps)</h3>
<p><code>SystemTime::now()</code> — can go backwards (NTP adjustments). Use for
recording event times, file modification times, expiry timestamps.
<code>SystemTime::UNIX_EPOCH</code> is the reference point.</p>

<h3>Sleeping</h3>
<p><code>std::thread::sleep(Duration::from_millis(100))</code> — blocks the current
OS thread. In async code use <code>tokio::time::sleep</code> instead (non-blocking,
doesn't hold up the executor).</p>"##,
            code: r#"use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn heavy_work(n: u64) -> u64 {
    // Simulate CPU-bound work
    (0..n).fold(0u64, |acc, x| acc.wrapping_add(x.wrapping_mul(x)))
}

fn main() {
    // --- Measuring elapsed time with Instant ---
    let start = Instant::now();
    let result = heavy_work(5_000_000);
    let elapsed: Duration = start.elapsed();

    println!("Result:       {}", result);
    println!("Elapsed:      {:?}", elapsed);           // e.g. 15.234ms
    println!("Elapsed ms:   {:.3}", elapsed.as_secs_f64() * 1_000.0);
    println!("Elapsed µs:   {}", elapsed.as_micros());

    // --- Duration arithmetic ---
    let d1 = Duration::from_millis(350);
    let d2 = Duration::from_millis(120);
    println!("\nd1 + d2  = {:?}", d1 + d2);
    println!("d1 - d2  = {:?}", d1.checked_sub(d2).unwrap());
    println!("d1 * 3   = {:?}", d1 * 3);

    // --- SystemTime for Unix timestamps ---
    let now_sys = SystemTime::now();
    let unix_secs = now_sys
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the Unix epoch")
        .as_secs();
    println!("\nUnix timestamp (s): {}", unix_secs);

    // --- Sleeping (commented to keep the demo fast) ---
    // std::thread::sleep(Duration::from_millis(50));
    // println!("Woke up after 50ms");
}
"#,
            notes: vec![
                "Always use `Instant` for measuring elapsed time — `SystemTime` can go backwards and is not suitable for performance measurement",
                "`Duration` stores seconds as `u64` + nanoseconds as `u32` internally — nanosecond precision with no floating-point rounding",
                "`elapsed()` is shorthand for `Instant::now().duration_since(start)` — it panics if the clock somehow moved backwards (extremely rare on modern hardware)",
                "For production async timeouts use `tokio::time::timeout(duration, future).await` — `thread::sleep` blocks the OS thread and starves other tasks",
            ],
        },
        Lesson {
            id: "logging-tracing",
            category: "Standard Library",
            title: "Logging with the `log` crate (and tracing)",
            description: r##"<p>The Rust standard library has <code>eprintln!</code> for quick debug prints.
Production code needs structured, level-filtered logging.</p>

<h3>The <code>log</code> crate — logging facade</h3>
<p><code>log</code> defines the API: <code>trace!</code>, <code>debug!</code>,
<code>info!</code>, <code>warn!</code>, <code>error!</code>.<br>
A separate <b>backend</b> must be initialized at startup. Common choices:</p>
<ul>
  <li><code>env_logger</code> — reads <code>RUST_LOG=info</code> from the environment;
      minimal setup, great for CLIs</li>
  <li><code>tracing</code> + <code>tracing-subscriber</code> — structured, async-aware,
      supports JSON output and distributed tracing</li>
</ul>

<h3>tracing — structured and async-first</h3>
<p><code>tracing</code> adds <b>spans</b> (time ranges with attached context) on top of
events. Spans carry context through <code>.await</code> boundaries — crucial for
debugging async code:</p>
<pre><code>#[tracing::instrument]
async fn handle_request(user_id: u64) {
    tracing::info!(user_id, "handling request");
    // output: {"level":"INFO","user_id":42,"message":"handling request"}
}</code></pre>

<h3>vs Node.js</h3>
<p>Similar to using <code>winston</code> or <code>pino</code> with a transport backend.
Rust's advantage: log levels not reached are <em>eliminated at compile time</em> in
release builds when configured — zero runtime overhead for disabled levels.</p>

<h3>Quick setup</h3>
<pre><code>[dependencies]
log        = "0.4"
env_logger = "0.11"</code></pre>
<pre><code>fn main() {
    env_logger::init(); // reads RUST_LOG env var
    log::info!("server starting on port {}", 8080);
}</code></pre>"##,
            code: r#"// Simulating structured logging with macros.
// In production: add `log = "0.4"` + `env_logger = "0.11"` to Cargo.toml,
// then call `env_logger::init()` at the start of main().

macro_rules! log_info  { ($($t:tt)*) => { eprintln!("[INFO]  {}", format!($($t)*)) } }
macro_rules! log_warn  { ($($t:tt)*) => { eprintln!("[WARN]  {}", format!($($t)*)) } }
macro_rules! log_error { ($($t:tt)*) => { eprintln!("[ERROR] {}", format!($($t)*)) } }
macro_rules! log_debug { ($($t:tt)*) => { eprintln!("[DEBUG] {}", format!($($t)*)) } }

fn connect(url: &str) -> Result<(), String> {
    log_info!("Connecting to {}", url);
    if url.is_empty() {
        log_error!("URL is empty");
        return Err("empty URL".into());
    }
    log_debug!("Resolved host, opening socket");
    log_info!("Connection established");
    Ok(())
}

fn process_batch(items: &[&str]) {
    if items.is_empty() {
        log_warn!("process_batch called with empty slice — nothing to do");
        return;
    }
    log_info!("Processing {} items", items.len());
    for item in items {
        log_debug!("  processing item: {}", item);
    }
    log_info!("Batch complete");
}

fn main() {
    log_info!("Application starting");

    connect("postgres://localhost/mydb").expect("DB connection failed");
    connect("").unwrap_or_else(|e| log_error!("Caught: {}", e));

    process_batch(&["order-1", "order-2", "order-3"]);
    process_batch(&[]);

    // With `tracing` you'd write:
    //   tracing::info!(batch_size = 3, "processing batch");
    // and get structured JSON:
    //   {"level":"INFO","batch_size":3,"message":"processing batch"}
    log_info!("Shutdown complete");
}
"#,
            notes: vec![
                "`RUST_LOG=debug cargo run` controls log levels at runtime with `env_logger` — no code changes needed to change verbosity",
                "The `log` crate's macros compile down to a no-op when the level is disabled — zero overhead in release builds for filtered-out levels",
                "`tracing` spans carry context through `.await` boundaries — essential for correlating log lines across concurrent async tasks",
                "`#[tracing::instrument]` auto-creates a span for a function, automatically recording all parameters as span fields",
            ],
        },
    ]
}
