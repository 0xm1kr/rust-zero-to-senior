use super::Lesson;

pub fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            id: "frameworks",
            category: "Ecosystem",
            title: "Web Frameworks: axum, actix-web, Rocket, warp",
            description: r#"<p>Rust has several production-quality web frameworks. Each makes different trade-offs; knowing the landscape is expected in senior interviews.</p>

<h3>axum <b>★ recommended default</b></h3>
<ul>
  <li>Built on tokio + hyper + tower — shares the ecosystem with every tower-compatible service</li>
  <li>Extractors are typed: <code>Path&lt;(u64,)&gt;</code>, <code>Json&lt;T&gt;</code>, <code>Query&lt;T&gt;</code> — compile-time validation</li>
  <li>Zero magic macros; explicit, readable, testable</li>
  <li>Best documentation and largest community post-2022</li>
</ul>

<h3>actix-web</h3>
<ul>
  <li>Mature (2018), extremely fast, has its own actor-influenced threading model</li>
  <li>Slightly harder to compose with tower middleware</li>
  <li>Pick when you need maximum raw throughput or are maintaining an existing actix codebase</li>
</ul>

<h3>Rocket</h3>
<ul>
  <li>Most ergonomic for beginners — macros handle routing: <code>#[get("/user/&lt;id&gt;")]</code></li>
  <li>Moved to async in v0.5; production-ready but more magic than axum</li>
  <li>Good fit if your team comes from a Rails/Django background and values convention over configuration</li>
</ul>

<h3>warp</h3>
<ul>
  <li>Filter combinator pattern: routes composed with <code>.and()</code> / <code>.or()</code></li>
  <li>Elegant in theory, but type errors can be cryptic; community adoption declined after axum launched</li>
  <li>Niche: purely functional-style route composition</li>
</ul>

<h3>Newer entrants</h3>
<p><b>poem</b> and <b>salvo</b> are gaining traction. <b>ntex</b> is an actix-web fork with continued active development. For most greenfield projects: <b>default to axum</b>.</p>

<h3>Interview answer</h3>
<p>"I default to axum because it composes naturally with the tower ecosystem, extractors are compile-time type-safe, and the community is largest. I'd reach for actix-web for a performance-critical legacy service."</p>"#,
            code: r##"// "Routes as data" — the pattern every Rust web framework builds on.
// axum::Router, actix-web's App, and Rocket's routes![] all assemble
// routes at startup; this shows the concept in pure stdlib.

#[derive(Debug, Clone)]
struct Route {
    method:      &'static str,
    path:        &'static str,
    description: &'static str,
}

struct App {
    name:   &'static str,
    routes: Vec<Route>,
}

impl App {
    fn new(name: &'static str) -> Self {
        App { name, routes: Vec::new() }
    }

    fn get(mut self, path: &'static str, description: &'static str) -> Self {
        self.routes.push(Route { method: "GET", path, description });
        self
    }

    fn post(mut self, path: &'static str, description: &'static str) -> Self {
        self.routes.push(Route { method: "POST", path, description });
        self
    }

    fn delete(mut self, path: &'static str, description: &'static str) -> Self {
        self.routes.push(Route { method: "DELETE", path, description });
        self
    }

    fn print_routes(&self) {
        println!("=== {} routes ===", self.name);
        for r in &self.routes {
            println!("  {:<7} {:<22} — {}", r.method, r.path, r.description);
        }
    }
}

fn main() {
    let app = App::new("users-service")
        .get("/health",         "readiness probe")
        .get("/users",          "list all users")
        .post("/users",         "create a user")
        .get("/users/:id",      "get user by id")
        .post("/users/:id",     "update user")
        .delete("/users/:id",   "delete user");

    app.print_routes();

    println!("\nTotal routes: {}", app.routes.len());
}
"##,
            notes: vec![
                "Default to axum for new projects — tower compatibility, typed extractors, and best community support.",
                "actix-web is still excellent for raw throughput; choose it when benchmarks matter more than ecosystem fit.",
                "Rocket is the most beginner-friendly but the macro magic can hide errors until runtime in edge cases.",
                "warp is largely superseded by axum; avoid for new projects unless you need its specific combinator style.",
                "All major Rust web frameworks are production-ready — the choice is about ecosystem fit, not safety.",
            ],
        },
        Lesson {
            id: "what-rust-is-great-at",
            category: "Ecosystem",
            title: "What Rust Is Great At",
            description: r#"<p>Rust's zero-cost abstractions, memory safety without GC, and predictable latency make it the best tool for specific problem classes. Knowing these is part of senior-level fluency.</p>

<h3>Where Rust dominates</h3>
<ul>
  <li><b>Systems / OS</b> — the Linux kernel now accepts Rust drivers; Windows kernel team exploring Rust</li>
  <li><b>Embedded / no_std</b> — runs on microcontrollers with 16 KB RAM; biggest growth area by number of new projects</li>
  <li><b>CLI tools</b> — ripgrep, fd, bat, exa, tokei, gitui — consistently faster than GNU equivalents</li>
  <li><b>Network services</b> — Discord's Read States service (millions of concurrent users), Cloudflare Workers runtime</li>
  <li><b>Web backends</b> — axum / actix-web rival Go and Java throughput at lower memory usage</li>
  <li><b>Databases</b> — SurrealDB, TiKV (TiDB storage layer), Databend, RocksDB bindings</li>
  <li><b>Browsers</b> — Firefox Stylo (CSS engine), WebRender (GPU compositor), Servo project ongoing</li>
  <li><b>Build tooling</b> — Deno (V8 embedding), Rolldown (next-gen Vite bundler), Biome (Rome fork), SWC</li>
  <li><b>AI infrastructure</b> — Hugging Face tokenizers, Candle ML framework, LanceDB vector store</li>
  <li><b>Blockchain</b> — Solana, Polkadot (Substrate), Near Protocol, Aptos — Rust is the dominant chain language</li>
</ul>

<h3>When NOT to pick Rust (yet)</h3>
<ul>
  <li>Trivial CRUD where developer iteration speed matters more than runtime performance</li>
  <li>Teams with no Rust experience and a tight deadline — the learning curve is real and front-loaded</li>
  <li>Large monorepos without a good incremental build setup — compile times add up</li>
</ul>

<h3>The interview answer</h3>
<p>Rust is right when you need C/C++ performance, need to eliminate memory bugs at the language level, or are building infrastructure that runs at massive scale. It is <em>not</em> the right choice just because it's trending. TypeScript is still the right tool for most product CRUD APIs.</p>"#,
            code: r##"fn main() {
    let strengths: &[(&str, &str)] = &[
        ("Systems / OS",    "Linux kernel drivers, Windows kernel (in progress)"),
        ("Embedded",        "no_std, zero-cost abstractions, no GC pause"),
        ("CLI tools",       "ripgrep, fd, bat, exa — faster than GNU equivalents"),
        ("Network svc",     "Discord backend, Cloudflare Workers runtime"),
        ("Web backends",    "axum/actix-web rival Go/Java throughput"),
        ("Databases",       "SurrealDB, TiKV, Databend, RocksDB bindings"),
        ("Browsers",        "Firefox Stylo + WebRender, Servo project"),
        ("Build tooling",   "Deno, Rolldown, Biome, SWC — replacing JS toolchain"),
        ("AI infra",        "HuggingFace tokenizers, Candle ML, LanceDB"),
        ("Blockchain",      "Solana, Polkadot, Near — dominant chain language"),
    ];

    println!("Where Rust shines:\n");
    for (domain, example) in strengths {
        println!("  {:<18}  {}", domain, example);
    }

    println!("\nWhen NOT to pick Rust:");
    println!("  - Trivial CRUD where developer iteration > runtime performance");
    println!("  - Teams with no Rust experience and a tight deadline");
    println!("  - Large repos where compile times hurt the feedback loop");
    println!("  - Anywhere TypeScript + Node.js is already working well");
}
"##,
            notes: vec![
                "Rust is now a first-class language in the Linux kernel — the first new kernel language in 30+ years.",
                "The most impactful use case for most engineers: replacing performance-critical Node.js scripts or services.",
                "Embedded / no_std is Rust's fastest-growing segment by number of new crates and contributors.",
                "The 'rewrite it in Rust' meme is real: ripgrep, fd, bat, exa, Biome all beat their predecessors in benchmarks.",
                "Rust is not the right tool for every job — knowing when NOT to use it is also a senior-level skill.",
            ],
        },
    ]
}
