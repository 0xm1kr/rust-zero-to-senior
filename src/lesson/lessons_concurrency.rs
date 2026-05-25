use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "threads",
            category: "Concurrency",
            title: "Threads with std::thread",
            description: r##"<p><code>std::thread::spawn</code> takes a closure, starts an OS thread, and
returns a <code>JoinHandle&lt;T&gt;</code>. Call <code>.join()</code> to block until the
thread finishes and collect its return value (or its panic as an <code>Err</code>).</p>

<h3>Key differences from Node.js workers</h3>
<ul>
  <li>One OS thread per spawn — heavier than goroutines; for thousands of
      concurrent tasks use async/tokio instead</li>
  <li>No shared event loop — each thread runs independently on the OS scheduler</li>
  <li>Captured variables must be <code>move</code>d into the closure — ownership
      transfers to the new thread, enforced at compile time</li>
  <li>Panics in a spawned thread are caught: <code>.join()</code> returns
      <code>Err</code> rather than crashing the process</li>
</ul>

<h3>When to use threads vs async</h3>
<p><b>Threads</b> — CPU-bound work (parsing, crypto, image processing) where you
want true parallelism. <b>Async/tokio</b> — I/O-bound work (HTTP, DB, file) where
you need to handle thousands of concurrent connections cheaply.</p>"##,
            code: r#"use std::thread;

fn main() {
    // Spawn 5 workers; collect their JoinHandles
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                // `move` captures `i` by value — mandatory for thread closures
                println!("Worker {} running on {:?}", i, thread::current().id());
                i * i  // return value propagates through join()
            })
        })
        .collect();

    // Wait for every worker; unwrap the Result (Err = thread panicked)
    let results: Vec<u32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    println!("Squares: {:?}", results);
    println!("All workers finished.");
}
"#,
            notes: vec![
                "`thread::spawn` returns `JoinHandle<T>` — dropping it detaches the thread (it keeps running but you can no longer join it)",
                "Always use `move` closures with `spawn` — the compiler will reject captures by reference since the thread may outlive the stack frame",
                "For CPU-bound parallel work, `std::thread` gives true parallelism (one OS thread per core); pair with `mpsc` channels to collect results",
                "Panics inside spawned threads are caught: `join()` returns `Err(Box<dyn Any>)` — the process does NOT crash",
            ],
        },
        Lesson {
            id: "channels",
            category: "Concurrency",
            title: "Channels (mpsc)",
            description: r##"<p><code>std::sync::mpsc::channel()</code> returns a <b>(Sender, Receiver)</b> pair.
<b>mpsc</b> = <em>multiple-producer, single-consumer</em>. Clone the <code>Sender</code>
to give multiple threads the same channel; there is always exactly one <code>Receiver</code>.</p>

<h3>Ownership semantics</h3>
<p>Sending <em>moves</em> the value — the sending thread gives up ownership. The receiving
thread owns it. No shared references, no locks, no data races by design. Compare to
Node.js <code>worker.postMessage()</code>, but enforced at compile time.</p>

<h3>Bounded vs unbounded</h3>
<ul>
  <li><code>channel()</code> — unbounded buffer; <code>send</code> never blocks the sender</li>
  <li><code>sync_channel(n)</code> — bounded to <em>n</em> slots; sender blocks when full
      (built-in back-pressure)</li>
  <li><code>sync_channel(0)</code> — rendezvous channel; sender blocks until receiver is ready</li>
</ul>

<h3>Clean shutdown</h3>
<p>When all <code>Sender</code>s drop, the <code>Receiver</code> iterator ends automatically —
no sentinel values or extra signaling needed.</p>"##,
            code: r#"use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<u32>();

    // Producer thread — computes squares, sends them
    thread::spawn(move || {
        for i in 1..=5 {
            let square = i * i;
            println!("Sending: {}", square);
            tx.send(square).unwrap(); // Err if receiver was dropped
        }
        // tx drops here → rx iterator will end
    });

    // Main thread is the single consumer
    // Iterating `rx` blocks until the next message or channel close
    for val in rx {
        println!("Received: {}", val);
    }
    println!("Channel closed — all producers finished.");

    // Multiple producers example (not run, shown for reference):
    // let tx2 = tx.clone(); // give second thread its own sender
}
"#,
            notes: vec![
                "`Sender<T>` implements `Clone` — each clone gets independent sending rights; all share the same channel",
                "`tx.send(val)` returns `Err(val)` if the receiver has been dropped — always handle the Result in production",
                "`for val in rx` is idiomatic: blocks on each iteration, stops cleanly when all Senders are dropped",
                "Use `sync_channel(capacity)` when you need back-pressure — the bounded buffer prevents a fast producer from overwhelming a slow consumer",
            ],
        },
        Lesson {
            id: "shared-state",
            category: "Concurrency",
            title: "Arc<Mutex<T>> for Shared State",
            description: r##"<p>When multiple threads need the <em>same mutable value</em>, wrap it in
<code>Arc&lt;Mutex&lt;T&gt;&gt;</code>:</p>
<ul>
  <li><b><code>Arc&lt;T&gt;</code></b> — <em>Atomically Reference-Counted</em> pointer;
      cheap to clone; allows shared ownership across threads</li>
  <li><b><code>Mutex&lt;T&gt;</code></b> — wraps the data; <code>.lock()</code> blocks
      until acquired and returns a <code>MutexGuard</code> that releases the lock on drop</li>
</ul>

<h3>Pattern</h3>
<pre><code>let shared = Arc::new(Mutex::new(0u32));
let clone  = Arc::clone(&amp;shared);         // cheap; increments atomic refcount
thread::spawn(move || {
    let mut guard = clone.lock().unwrap(); // blocks until lock available
    *guard += 1;
});  // guard drops here → lock released automatically</code></pre>

<h3>RwLock for read-heavy workloads</h3>
<p><code>Arc&lt;RwLock&lt;T&gt;&gt;</code>: multiple threads can hold a <em>read</em> lock
simultaneously; only one can hold the <em>write</em> lock. Use when reads vastly
outnumber writes.</p>

<h3>Deadlock warning</h3>
<p>Rust prevents <em>data races</em> but not <em>deadlocks</em>. Never call
<code>.lock()</code> while already holding the same lock on the same thread.</p>"##,
            code: r#"use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // --- Mutex: exclusive access ---
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter); // cheap atomic refcount increment
        handles.push(thread::spawn(move || {
            let mut guard = c.lock().unwrap(); // blocks; returns MutexGuard
            *guard += 1;
            // guard drops here → lock released before thread exits
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("Counter after 10 threads: {}", *counter.lock().unwrap()); // 10

    // --- RwLock: many readers, one writer ---
    let config = Arc::new(RwLock::new(vec!["timeout=30", "retry=3"]));

    let cfg_read = Arc::clone(&config);
    let reader = thread::spawn(move || {
        let r = cfg_read.read().unwrap(); // multiple readers can coexist
        println!("Config: {:?}", *r);
    });

    {
        let mut w = config.write().unwrap(); // exclusive; waits for all readers
        w.push("debug=true");
    }
    reader.join().unwrap();
    println!("Updated config: {:?}", *config.read().unwrap());
}
"#,
            notes: vec![
                "`Arc::clone` increments an atomic reference count — it does NOT deep-clone the data; the clone and original point to the same allocation",
                "`.lock()` returns `Err` only if another thread panicked while holding the lock (\"poisoned\" mutex) — `.unwrap()` is usually fine in application code",
                "The `MutexGuard` implements `Deref`/`DerefMut`, so use `*guard` to access the value",
                "For simple integer counters, `AtomicUsize` is faster than `Mutex<usize>` (no kernel lock, no blocking) — see the Thread Safety Patterns lesson",
            ],
        },
        Lesson {
            id: "send-sync",
            category: "Concurrency",
            title: "Send and Sync (Marker Traits)",
            description: r##"<p><code>Send</code> and <code>Sync</code> are <b>marker traits</b> — no methods,
just compile-time promises about thread safety:</p>
<ul>
  <li><b><code>Send</code></b>: it is safe to <em>move</em> a <code>T</code> to another thread</li>
  <li><b><code>Sync</code></b>: it is safe to <em>share</em> <code>&amp;T</code> across threads
      — i.e. <code>&amp;T: Send</code></li>
</ul>
<p>Most stdlib types are automatically <code>Send + Sync</code>. The compiler <em>derives</em>
these traits for structs whose fields are all <code>Send</code>/<code>Sync</code>.</p>

<h3>Notable non-Send / non-Sync types</h3>
<ul>
  <li><code>Rc&lt;T&gt;</code> — non-atomic refcount; sending it would corrupt the count
      without anyone noticing</li>
  <li><code>RefCell&lt;T&gt;</code> — runtime borrow checking uses a plain <code>Cell</code>
      internally, which is not thread-safe</li>
  <li><code>*mut T</code> / <code>*const T</code> (raw pointers) — opt out of all safety guarantees</li>
</ul>

<h3>Fearless Concurrency</h3>
<p>Rust catches data races <b>at compile time</b> through these markers. No ThreadSanitizer,
no runtime checks — the type system simply rejects programs that share non-thread-safe types
across threads. Compare to TypeScript, where nothing prevents passing a shared mutable object
across worker threads unsafely.</p>"##,
            code: r#"use std::sync::Arc;
use std::thread;

fn main() {
    // Arc<Vec<i32>> is Send + Sync: safe to share across threads.
    // Arc gives shared ownership; Vec<i32> itself is Send.
    let data = Arc::new(vec![10, 20, 30]);
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        // data_clone moved into the thread — Arc tracks the extra reference
        println!("Thread sees: {:?}", *data_clone);
        data_clone.iter().sum::<i32>() // return the sum
    });

    let sum = handle.join().unwrap();
    println!("Main still sees: {:?}", *data);
    println!("Sum computed by thread: {}", sum);

    // --- What happens with Rc instead of Arc ---
    // use std::rc::Rc;
    // let rc = Rc::new(42);
    // thread::spawn(move || println!("{rc}"));
    // ^^^ COMPILE ERROR: `Rc<i32>` cannot be sent between threads safely
    //     the trait `Send` is not implemented for `Rc<i32>`
}
"#,
            notes: vec![
                "The compiler automatically derives `Send`/`Sync` for structs whose fields are all `Send`/`Sync` — you rarely implement them manually",
                "To opt out (e.g. for a type wrapping a raw pointer), hold a `PhantomData<*mut T>` field or explicitly `impl !Send for MyType {}`",
                "`Arc<T>` is `Send + Sync` when `T: Send + Sync`; `Mutex<T>` is `Sync` even when `T: !Sync` — that's the whole point of the mutex",
                "This compile-time enforcement is unique to Rust — Go, C++, and TypeScript all have analogous concepts but enforce them at runtime or not at all",
            ],
        },
        Lesson {
            id: "async-await",
            category: "Concurrency",
            title: "async / await: The Idea",
            description: r##"<p><code>async fn</code> returns a <b>Future</b> — a value representing a
computation that hasn't finished (or even started) yet. Futures are <em>lazy</em>:
nothing runs until an <b>executor</b> polls the Future.</p>

<h3>Mental model: JS Promises vs Rust Futures</h3>
<ul>
  <li>JS: <code>async function</code> returns a Promise; the microtask queue drives it
      automatically</li>
  <li>Rust: <code>async fn</code> returns a <code>Future</code>; you choose and start
      the executor (<code>tokio</code>, <code>async-std</code>, <code>smol</code>)</li>
  <li><code>.await</code> suspends the current task and yields control back to the
      executor — other tasks run while we wait. Same idea as <code>await</code> in JS.</li>
  <li>Key difference: Rust Futures do <b>nothing</b> until polled (JS Promises
      start executing immediately on creation)</li>
</ul>

<h3>Typical pattern with tokio</h3>
<pre><code>#[tokio::main]
async fn main() {
    let raw  = fetch_user(1).await;    // suspends; executor runs other tasks
    let msg  = process(&amp;raw).await;   // suspends again
    println!("{msg}");
}</code></pre>

<h3>Under the hood</h3>
<p>The compiler rewrites <code>async fn</code> into an enum (state machine) that
implements <code>std::future::Future</code>. Each <code>.await</code> is a state
transition. This is a zero-cost abstraction — no heap allocation per suspension point.</p>
<p>Tokio is the industry standard for Rust web servers (Axum, Actix-web, Poem all
run on tokio). We cover it in the Web section. The code panel shows the
<em>synchronous equivalent</em> annotated with async comments.</p>"##,
            code: r#"// Synchronous simulation of an async "fetch then process" pipeline.
// In production you'd add `tokio = { version = "1", features = ["full"] }`
// and annotate main with #[tokio::main].

fn fetch_user(id: u32) -> String {
    // async version: `async fn fetch_user(id: u32) -> String`
    // at the `.await` point, the executor can run other tasks
    // while the network call is in flight.
    format!("{{\"id\":{},\"name\":\"Alice\",\"role\":\"admin\"}}", id)
}

fn process(raw: &str) -> String {
    // async version: could `.await` a DB lookup here
    let name_start = raw.find("\"name\":\"").map(|i| i + 8).unwrap_or(0);
    let name_end   = raw[name_start..].find('"').map(|i| i + name_start).unwrap_or(name_start);
    format!("Welcome, {}!", &raw[name_start..name_end])
}

fn main() {
    println!("=== Async/await flow (sync stand-in) ===\n");

    // In async code:  let raw = fetch_user(1).await;
    let raw = fetch_user(1);
    println!("Fetched:  {}", raw);

    // In async code:  let msg = process(&raw).await;
    let msg = process(&raw);
    println!("Processed: {}", msg);

    println!("\nWith tokio, each function would be `async fn` and each");
    println!("`.await` would yield the thread back to the executor,");
    println!("letting hundreds of other tasks make progress concurrently.");
}
"#,
            notes: vec![
                "A Future does NOTHING until polled — this is different from a JS Promise, which starts executing immediately on creation",
                "`tokio` is the runtime used by virtually every Rust web framework; add `#[tokio::main]` and `async fn main()` to get an async entry point",
                "`.await` is only valid inside `async fn` or `async {}` blocks — you cannot await in a regular synchronous function",
                "Under the hood the compiler transforms `async fn` into a state-machine enum — zero heap allocation per suspension, unlike green threads",
            ],
        },
        Lesson {
            id: "thread-safety-patterns",
            category: "Concurrency",
            title: "Thread Safety Patterns",
            description: r##"<p>A quick tour of the main concurrency patterns. Pick the simplest one that fits.</p>

<h3>1. Channels (message-passing — preferred when data flows one-way)</h3>
<p>Threads communicate by <b>sending values</b>. No shared state, no locks.
<code>mpsc::channel</code> for one-way pipelines; clone the <code>Sender</code> for
fan-in from multiple producers.</p>

<h3>2. <code>Arc&lt;T&gt;</code> — shared immutable data</h3>
<p>Share read-only data cheaply across threads. No locking needed — the data never
changes after construction. Perfect for configs, lookup tables, compiled regexes.</p>

<h3>3. <code>Arc&lt;Mutex&lt;T&gt;&gt;</code> — shared mutable data</h3>
<p>When you truly must mutate from multiple threads. The lock is released when
the <code>MutexGuard</code> drops. Use <code>RwLock</code> when reads dominate.</p>

<h3>4. Atomic types — lock-free counters and flags</h3>
<p><code>AtomicUsize</code>, <code>AtomicBool</code>, <code>AtomicI32</code>, etc.
in <code>std::sync::atomic</code>. Single-integer updates with no kernel involvement —
much faster than <code>Mutex</code> for counters and shutdown flags.</p>
<ul>
  <li><code>Ordering::Relaxed</code> — fastest; atomicity only, no cross-operation ordering</li>
  <li><code>Ordering::SeqCst</code> — strongest; total order across all atomic operations</li>
</ul>

<h3>The Rust concurrency mantra</h3>
<p><em>"Share memory by communicating; communicate by sharing memory — Rust gives you
both, safely."</em> Channels avoid shared state entirely; <code>Arc&lt;Mutex&gt;</code>
makes shared state safe. Start with channels; reach for atomics or Mutex only when needed.</p>"##,
            code: r#"use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;

fn demo_atomic_counter() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // fetch_add: atomic read-modify-write, no Mutex needed
            c.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for h in handles { h.join().unwrap(); }
    // SeqCst fence: ensures we see all prior writes
    println!("Atomic counter (100 threads): {}", counter.load(Ordering::SeqCst));
}

fn demo_channel_pipeline() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        for i in 1..=4 {
            tx.send(format!("task-{}", i)).unwrap();
        }
    });

    for msg in rx {
        println!("Processed: {}", msg);
    }
}

fn main() {
    println!("=== Lock-free atomic counter ===");
    demo_atomic_counter();

    println!("\n=== Channel pipeline ===");
    demo_channel_pipeline();
}
"#,
            notes: vec![
                "`AtomicBool` is ideal for shutdown flags: `running.store(false, Ordering::Release)` in a signal handler, `running.load(Ordering::Acquire)` in the loop",
                "`Ordering::Relaxed` is safe for independent counters; use `Release`/`Acquire` pairs when one thread must see the side-effects of another before the atomic op",
                "Prefer channels over `Arc<Mutex<T>>` when data flows directionally — channels compose better, are easier to reason about, and avoid deadlock risk",
                "For a global singleton (config, logger), use `std::sync::OnceLock<T>` (stable since 1.70) instead of a `Mutex<Option<T>>`",
            ],
        },
    ]
}
