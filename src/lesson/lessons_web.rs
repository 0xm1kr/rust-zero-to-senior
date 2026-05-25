use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "http-server",
            category: "Web Development",
            title: "An HTTP Server with std::net (and axum)",
            description: r#"<p>Rust's stdlib gives you <code>std::net::TcpListener</code> — raw TCP. You <em>can</em> hand-roll HTTP/1.1, but header parsing, keep-alive, chunked encoding, and TLS take weeks. In production the community uses a layered stack.</p>

<h3>The production stack</h3>
<ul>
  <li><b>tokio</b> — async runtime: event loop + thread pool</li>
  <li><b>hyper</b> — battle-tested HTTP/1.1 and HTTP/2 implementation</li>
  <li><b>tower</b> — <code>Service</code> / <code>Layer</code> middleware model</li>
  <li><b>axum</b> — ergonomic router and typed extractors built on all three</li>
</ul>

<h3>Minimal axum app (needs tokio — not runnable in Playground)</h3>
<pre><code>use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn hello() -&gt; &amp;'static str { "Hello!" }

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}</code></pre>

<p>Compare to Express: <code>app.get('/', (req, res) =&gt; res.send('hi'))</code>. The axum version uses typed extractors — <code>Path&lt;(u64,)&gt;</code>, <code>Json&lt;T&gt;</code>, <code>Query&lt;T&gt;</code> — checked at compile time. No runtime surprises on missing params or malformed bodies.</p>
<p>The runnable code below implements a <b>synchronous struct-based router</b> — same mental model, pure stdlib.</p>"#,
            code: r##"use std::collections::HashMap;

struct Request<'a> {
    method: &'a str,
    path:   &'a str,
}

struct Response {
    status: u16,
    body:   &'static str,
}

type Handler = fn() -> Response;

struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn get(&mut self, path: &str, h: Handler) {
        self.routes.insert(format!("GET {path}"), h);
    }

    fn handle(&self, req: &Request<'_>) -> Response {
        let key = format!("{} {}", req.method, req.path);
        match self.routes.get(&key) {
            Some(h) => h(),
            None    => Response { status: 404, body: "Not Found" },
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/",       || Response { status: 200, body: "Hello, World!" });
    router.get("/health", || Response { status: 200, body: "{\"status\":\"ok\"}" });

    let reqs = [
        Request { method: "GET", path: "/" },
        Request { method: "GET", path: "/health" },
        Request { method: "GET", path: "/missing" },
    ];

    for r in &reqs {
        let resp = router.handle(r);
        println!("{} {} => {} {}", r.method, r.path, resp.status, resp.body);
    }
}
"##,
            notes: vec![
                "In production Rust, axum is the standard choice for HTTP servers (2024+).",
                "axum routes are type-checked at compile time — wrong extractor types are a compile error, not a runtime panic.",
                "The tower Service trait is the building block for all middleware; axum just adds routing on top.",
                "std::net::TcpListener is useful for protocol experiments and learning, not production HTTP.",
                "TypeScript analogy: axum is Fastify/Express, hyper is Node's built-in http module, tokio is libuv.",
            ],
        },
        Lesson {
            id: "http-client",
            category: "Web Development",
            title: "HTTP Client (reqwest)",
            description: r#"<p><code>reqwest</code> is the de-facto HTTP client crate. It runs on top of tokio and hyper. Async by default; <code>reqwest::blocking</code> for sync code.</p>

<h3>Async JSON fetch</h3>
<pre><code>use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User { id: u64, name: String }

#[tokio::main]
async fn main() -&gt; reqwest::Result&lt;()&gt; {
    let user: User = reqwest::get("https://jsonplaceholder.typicode.com/users/1")
        .await?
        .json::&lt;User&gt;()
        .await?;
    println!("{:?}", user);
    Ok(())
}</code></pre>

<h3>Blocking API (no runtime needed)</h3>
<pre><code>let resp = reqwest::blocking::get("https://example.com/api/data")?
    .json::&lt;MyStruct&gt;()?;</code></pre>

<h3>Custom client with headers and timeout</h3>
<pre><code>let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()?;
let resp = client
    .get("https://api.example.com/data")
    .bearer_auth("my-token")
    .send()
    .await?
    .json::&lt;Data&gt;()
    .await?;</code></pre>

<p>Compare to <code>fetch()</code> in JS/TS: reqwest is strongly typed — the response is deserialized directly into your struct via serde. No <code>as unknown as MyType</code> casting. The compiler rejects mismatched field types at build time.</p>
<p>The runnable code simulates the <b>shape</b> of an HTTP client response using hand-written structs.</p>"#,
            code: r##"// Simulates what reqwest + serde_json would give you automatically.
// In production: reqwest::get(url).await?.json::<User>().await?

#[derive(Debug)]
struct User {
    id:    u64,
    name:  String,
    email: String,
}

#[derive(Debug)]
struct ApiError {
    status:  u16,
    message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {} — {}", self.status, self.message)
    }
}

/// Pretend this made a network request and deserialized the JSON body.
fn fetch_user(url: &str) -> User {
    println!("GET {url}");
    User {
        id:    42,
        name:  "Ada Lovelace".to_string(),
        email: "ada@example.com".to_string(),
    }
}

fn fetch_user_result(url: &str, succeed: bool) -> Result<User, ApiError> {
    println!("GET {url}");
    if succeed {
        Ok(User {
            id:    1,
            name:  "Grace Hopper".to_string(),
            email: "grace@example.com".to_string(),
        })
    } else {
        Err(ApiError { status: 404, message: "user not found".to_string() })
    }
}

fn main() {
    let user = fetch_user("https://api.example.com/users/42");
    println!("{:?}\n", user);

    match fetch_user_result("https://api.example.com/users/1", true) {
        Ok(u)  => println!("Found: {} <{}>", u.name, u.email),
        Err(e) => println!("Error: {e}"),
    }

    match fetch_user_result("https://api.example.com/users/999", false) {
        Ok(u)  => println!("Found: {}", u.name),
        Err(e) => println!("Error: {e}"),
    }
}
"##,
            notes: vec![
                "reqwest is the most popular HTTP client in the Rust ecosystem — nearly every project uses it.",
                "Use reqwest::blocking for scripts and tests; async reqwest for production services under tokio.",
                "reqwest + serde + #[derive(Deserialize)] gives you typed HTTP responses with near-zero boilerplate.",
                "Always set a timeout on your client builder — the default has no timeout and can block forever.",
                "TypeScript analogy: reqwest is axios, serde_json is JSON.parse() — but type-safe at compile time.",
            ],
        },
        Lesson {
            id: "middleware",
            category: "Web Development",
            title: "Middleware with Tower",
            description: r#"<p>Tower's <code>Service</code> trait is the backbone of axum middleware. A <code>Service</code> takes a request and returns a future of a response. A <code>Layer</code> wraps one <code>Service</code> in another — composing cross-cutting concerns without touching handler logic.</p>

<h3>axum + tower-http middleware stack</h3>
<pre><code>use tower_http::{
    trace::TraceLayer,
    timeout::TimeoutLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
};
use std::time::Duration;

let app = Router::new()
    .route("/", get(handler))
    .layer(CompressionLayer::new())
    .layer(CorsLayer::permissive())
    .layer(TimeoutLayer::new(Duration::from_secs(30)))
    .layer(TraceLayer::new_for_http()); // outermost = first to run</code></pre>

<p>Layers run in <b>reverse registration order</b>: the last <code>.layer()</code> call is outermost and sees the request first. Compare to Express: <code>app.use(cors())</code> — same idea, but typed and compile-checked end-to-end.</p>

<h3>Custom async middleware (axum)</h3>
<pre><code>async fn auth_middleware(
    State(db): State&lt;Db&gt;,
    req: Request,
    next: Next,
) -&gt; Response {
    if !is_authenticated(&amp;req, &amp;db).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(req).await
}

let app = Router::new()
    .route_layer(from_fn_with_state(db, auth_middleware));</code></pre>

<p>The runnable code demonstrates <b>synchronous function-composition middleware</b> — the same wrapping concept without async.</p>"#,
            code: r##"fn with_logging<F>(name: &'static str, handler: F) -> impl Fn(&str) -> String
where
    F: Fn(&str) -> String,
{
    move |req: &str| {
        println!("[LOG] {name} <- {req}");
        let resp = handler(req);
        println!("[LOG] {name} -> {resp}");
        resp
    }
}

fn with_auth<F>(handler: F) -> impl Fn(&str) -> String
where
    F: Fn(&str) -> String,
{
    move |req: &str| {
        if req.contains("token=secret") {
            handler(req)
        } else {
            "401 Unauthorized".to_string()
        }
    }
}

fn with_timing<F>(handler: F) -> impl Fn(&str) -> String
where
    F: Fn(&str) -> String,
{
    move |req: &str| {
        let start = std::time::Instant::now();
        let resp  = handler(req);
        println!("[TIME] {}µs", start.elapsed().as_micros());
        resp
    }
}

fn greet(req: &str) -> String {
    format!("200 OK: hello (req={})", req)
}

fn main() {
    // Compose: timing → logging → auth → greet  (outermost = timing)
    let app = with_timing(with_logging("greet", with_auth(greet)));

    println!("=== Request without token ===");
    let r = app("GET /greet");
    println!("Response: {r}\n");

    println!("=== Request with token ===");
    let r = app("GET /greet?token=secret");
    println!("Response: {r}");
}
"##,
            notes: vec![
                "Tower's Service/Layer is universal: axum, hyper, tonic (gRPC), and tower-http all share the same traits.",
                "tower-http provides production-ready layers out of the box: tracing, timeout, CORS, compression, auth.",
                "Layers run outermost-first; the last .layer() call wraps everything and sees the request first.",
                "For custom middleware, axum's from_fn / from_fn_with_state is the simplest path — no Service impl needed.",
                "TypeScript analogy: Tower layers are Express/Koa middleware, but the entire stack is type-checked.",
            ],
        },
        Lesson {
            id: "graceful-shutdown",
            category: "Web Development",
            title: "Production HTTP: Graceful Shutdown",
            description: r#"<p>A production service must handle <code>SIGTERM</code> without dropping in-flight requests. Pattern: stop accepting new connections → drain active handlers → exit cleanly.</p>

<h3>axum graceful shutdown</h3>
<pre><code>use tokio::signal;

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
    // On Unix, also catch SIGTERM (sent by Kubernetes, systemd, etc.):
    // signal::unix::signal(SignalKind::terminate())?.recv().await;
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}</code></pre>

<h3>Production checklist</h3>
<ul>
  <li><b>Timeout</b> — add <code>TimeoutLayer</code> so stuck handlers don't block the drain indefinitely</li>
  <li><b>Readiness probe</b> — return 503 once shutdown is triggered so load balancers stop routing traffic</li>
  <li><b>Structured logging</b> — <code>tracing</code> crate + <code>tracing-subscriber</code>; emit JSON in production</li>
  <li><b>Error handling</b> — implement <code>IntoResponse</code> for your error type; never <code>panic!</code> in handlers</li>
  <li><b>Metrics</b> — <code>metrics</code> crate facade or Prometheus via <code>axum-prometheus</code></li>
</ul>

<p>Compare to Node.js: <code>process.on('SIGTERM', () =&gt; server.close(cb))</code>. Same concept, but Rust's type system ensures you can't accidentally drop a shared request context mid-flight.</p>
<p>The runnable code simulates a graceful drain using <code>AtomicBool</code> and <code>AtomicI32</code>.</p>"#,
            code: r##"use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let shutdown  = Arc::new(AtomicBool::new(false));
    let in_flight = Arc::new(AtomicI32::new(5));

    // Spawn workers simulating in-flight request handlers.
    let handles: Vec<_> = (0..5).map(|i| {
        let in_flight = Arc::clone(&in_flight);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(30 + i * 20));
            let remaining = in_flight.fetch_sub(1, Ordering::SeqCst) - 1;
            println!("  request {i} finished ({remaining} remaining)");
        })
    }).collect();

    // Trigger shutdown signal.
    thread::sleep(Duration::from_millis(10));
    println!("shutting down...");
    shutdown.store(true, Ordering::SeqCst);

    // Drain: wait until all in-flight requests complete.
    while in_flight.load(Ordering::SeqCst) > 0 {
        println!("  waiting — {} in flight", in_flight.load(Ordering::SeqCst));
        thread::sleep(Duration::from_millis(30));
    }

    for h in handles { h.join().unwrap(); }
    println!("drained, exiting");
}
"##,
            notes: vec![
                "axum::serve(...).with_graceful_shutdown(signal) handles the drain loop for you in one call.",
                "Always catch both SIGTERM (Kubernetes, systemd) and SIGINT (developer Ctrl+C).",
                "Set a maximum drain timeout — if requests don't finish in e.g. 30s, force-exit to avoid zombie pods.",
                "Flip a readiness flag immediately on shutdown so the load balancer stops sending new traffic first.",
                "The tracing crate is the standard structured-logging solution in Rust; pair with tracing-subscriber for JSON output.",
            ],
        },
    ]
}
