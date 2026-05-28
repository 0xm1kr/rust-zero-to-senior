# 404skills — Rust

A self-contained, fullstack tutorial app for learning Rust end-to-end,
built for engineers who already know Node/TypeScript and need to be
**interview-ready for a senior Rust role**.

The curriculum walks the language from scratch, then layers on
ownership, lifetimes, traits, async/await, memory & performance, senior
pitfalls, and the algorithms that show up in coding interviews. Each
lesson has a TS comparison where helpful, an editable example, and
compiles/runs your code for real, not simulated.

The backend is written in idiomatic Rust using
**[axum](https://docs.rs/axum) + [tokio](https://docs.rs/tokio) + [reqwest](https://docs.rs/reqwest)**
plus a handful of small, well-known crates — read its source as a
second tutorial. The frontend is plain HTML / CSS / vanilla JS, no
build step.

```
┌──────────────────────────────────────────────────────────────────┐
│  Sidebar lessons          │  Lesson description + key takeaways  │
│  Search / progress bar    │                                      │
│                           │  ┌────────────────────────────────┐  │
│   ✓ Welcome & Setup       │  │  Editable Rust code            │  │
│   ○ Variables & Mutab…    │  │  (real rustc on backend)       │  │
│   ○ Ownership Rules       │  └────────────────────────────────┘  │
│   ○ ...                   │  ┌────────────────────────────────┐  │
│                           │  │  Output / stderr / errors      │  │
│                           │  └────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

## Run it

You need [Rust 1.75+](https://rustup.rs/) installed (`rustup default
stable` is fine). The curriculum uses async/await (stable since 1.39),
`let … else` (1.65), GATs (1.65), and a few 2021-edition idioms.
Latest stable is recommended.

```bash
cargo run
# Rust tutorial running at http://localhost:8080
```

Open <http://localhost:8080> in your browser.

### Optional flags

```bash
cargo run -- -addr :9000      # bind to a different port
```

### Ask AI (optional)

Each lesson has a floating **Ask AI** button that opens a chat panel. The
question is sent to an LLM along with the current lesson's title,
description, code example, key takeaways, AND whatever you've typed in
the editor, so you can ask things like *"why won't this compile?"*
and get a specific answer.

The server auto-detects which provider to use from your environment, in
this priority order: **Google → Anthropic → OpenAI**.

Set things up via a `.env` file. Copy the example and fill in one key:

```bash
cp .env.example .env
# edit .env and paste your key
cargo run
```

A `.env` file in the project root is loaded automatically on startup.
Get a free Gemini key at <https://aistudio.google.com/apikey>.

Real environment variables always win over the `.env` file, so you can
also do either of these without touching the file:

```bash
# One-shot for a single run:
GEMINI_API_KEY=AIza… cargo run

# Or shell-export:
export GEMINI_API_KEY=AIza…
cargo run
```

Other supported variables:

| Variable | Default | Notes |
| --- | --- | --- |
| `GEMINI_API_KEY` | _none_ | Google Gemini (preferred) |
| `GEMINI_MODEL` | `gemini-2.5-flash` | e.g. `gemini-2.5-pro` |
| `ANTHROPIC_API_KEY` | _none_ | Anthropic Claude |
| `ANTHROPIC_MODEL` | `claude-haiku-4-5` | |
| `OPENAI_API_KEY` | _none_ | OpenAI |
| `OPENAI_MODEL` | `gpt-4o-mini` | |

### Chat abuse protection

When AI chat is enabled on a public deployment, these limits apply per client IP
(Cloud Run sets `X-Forwarded-For` automatically; set `TRUST_PROXY=1` behind other
reverse proxies):

| Variable | Default | Notes |
| --- | --- | --- |
| `CHAT_RATE_PER_MIN` | `5` | Burst limit; set `0` to disable |
| `CHAT_RATE_DAILY` | `50` | Daily quota per IP; set `0` to disable |
| `CHAT_MAX_MESSAGE_CHARS` | `4000` | Max characters per message |
| `CHAT_MAX_CODE_CHARS` | `16000` | Max editor code sent with each turn |
| `CHAT_MAX_BODY_BYTES` | `65536` | Max JSON body size for `/api/chat` |

Exceeded limits return HTTP 429 with a `Retry-After` header. Limits are also
exposed via `GET /api/chat/status` under `limits`.

On startup the server logs which provider is active:

```
AI chat enabled via google (gemini-2.5-flash)
Rust tutorial listening on http://0.0.0.0:8080
```

If no key is set, the chat panel still opens but explains how to
configure one. Everything else in the app still works without an
API key.

Chat history is kept **per lesson, in localStorage**: switch lessons
and you get a fresh thread; the **trash icon** clears the current
thread. Press `Esc` or click the **×** to close the panel.

## Curriculum

The curriculum is "zero → interview-ready for a senior Rust role." It
assumes you already write Node/TypeScript at a senior level. Each
lesson is editable and compiles/runs for real on your machine.

The lessons are grouped into **16 sections**. The first 11 sections
teach the language end-to-end; the last 5 are the senior / interview
track.

### Foundations

| Section | Lessons |
| --- | --- |
| **Basics** | Welcome & Setup · Coming from TypeScript: Mental Model · Variables & Mutability · Basic Types & Conversions · Constants and Static |
| **Control Flow** | if, loop, while, for · match and Patterns · Functions & Return Values · Closures and Iterators · Option<T> and the ? Operator · Early Returns & Guard Clauses |
| **Ownership & Borrowing** | Ownership Rules · References & Borrowing · Move vs Copy vs Clone · Slices · String vs &str · Box / Rc / Arc |
| **Data Structures** | Vec · Arrays & Slices · HashMap & BTreeMap · Structs · Enums (ADTs) · Tuples & Destructuring |
| **Traits & Generics** | Methods via impl · Traits · Generic Functions & Bounds · Trait Objects (dyn) · Derivable Traits · The Iterator Trait · Monomorphization |
| **Errors** | Result<T,E> & ? · panic! vs Result · Custom Error Types · Error Handling Patterns |
| **Concurrency** | Threads · Channels (mpsc) · Arc<Mutex<T>> · Send / Sync · async / await: The Idea · Thread Safety Patterns |
| **Tooling & Packages** | Cargo · rustfmt & clippy · Modules & Visibility · Writing Tests · Benchmarks & Profiling |
| **Standard Library** | Files & io::Read/Write · Strings & Formatting · Collections Deep Dive · time: Duration & Instant · Logging with log/tracing |
| **Web Development** | HTTP Server (axum) · HTTP Client (reqwest) · Middleware with Tower · Graceful Shutdown |
| **Ecosystem** | Web Frameworks: axum, actix, Rocket, warp · What Rust Is Great At |

### Senior / Interview Track

| Section | Lessons |
| --- | --- |
| **Lifetimes** | Lifetime Annotations · Elision Rules · 'static & HRTB · PhantomData & Variance |
| **Memory & Performance** | Stack vs Heap · Zero-Cost Iterators · Reducing Allocations |
| **Senior Pitfalls** | Borrow-Checker Battles · Interior Mutability · Unsafe, Send/Sync, FFI |
| **Interview Algorithms** | Two Pointers · Sliding Window · Binary Search & Variants · Backtracking · Dynamic Programming · Graph Traversal (BFS/DFS) · Heaps & Top-K · Linked Lists in Rust · LRU Cache |
| **Interview Prep** | Senior Rust Interview Cheatsheet |

Progress is tracked in `localStorage`. Use the "Reset progress" button
in the sidebar to clear it.

## Project layout: read this as a second lesson

The codebase is organized by **domain**, not by technical layer. Each
module owns its types AND its access layer; the `api` module sits on
top and depends on all the domains, but no domain depends on HTTP.
`main.rs` is just the composition root.

```
404skills-rust/
├── Cargo.toml          # crate manifest + dependencies
├── Cargo.lock          # resolver output (committed for binaries)
├── src/
│   ├── main.rs         # composition root: load config, wire deps, start server
│   │
│   ├── config/         # .env loader
│   │   ├── mod.rs
│   │   └── dotenv.rs
│   │
│   ├── lesson/         # lesson domain. Lesson type + Repository port + catalog.
│   │   ├── mod.rs              # Lesson, Summary, Repository, InMemoryRepository
│   │   ├── catalog.rs          # composes per-category slices into the ordered Catalog
│   │   ├── lessons_basics.rs   # Foundations: Basics → Ecosystem (one file per category)
│   │   ├── lessons_control_flow.rs
│   │   ├── lessons_ownership.rs
│   │   ├── lessons_data_structures.rs
│   │   ├── lessons_traits_generics.rs
│   │   ├── lessons_errors.rs
│   │   ├── lessons_concurrency.rs
│   │   ├── lessons_tooling.rs
│   │   ├── lessons_stdlib.rs
│   │   ├── lessons_web.rs
│   │   ├── lessons_ecosystem.rs
│   │   ├── lessons_lifetimes.rs    # Senior track: Lifetimes → Interview Prep
│   │   ├── lessons_memory.rs
│   │   ├── lessons_pitfalls.rs
│   │   ├── lessons_algorithms.rs
│   │   └── lessons_interview_prep.rs
│   │
│   ├── runner/         # pluggable code sandbox
│   │   ├── mod.rs              # Runner trait + from_env selector
│   │   ├── local.rs            # `rustc` in a temp dir (dev default)
│   │   └── playground.rs       # POST to play.rust-lang.org (cloud default)
│   │
│   ├── tutor/          # LLM chat domain
│   │   ├── mod.rs
│   │   ├── service.rs          # Service + system-prompt builder + stripHTML
│   │   ├── provider.rs         # Provider trait + select_from_env
│   │   ├── httpx.rs            # shared post_json helper (DRY across providers)
│   │   ├── gemini.rs           # Google Gemini adapter
│   │   ├── anthropic.rs        # Anthropic Claude adapter
│   │   └── openai.rs           # OpenAI adapter
│   │
│   └── api/            # HTTP transport, no business logic here
│       ├── mod.rs              # Router, log middleware, AppState
│       ├── lessons.rs          # GET /api/lessons{,/:id}
│       ├── run.rs              # POST /api/run
│       ├── chat.rs             # POST /api/chat, GET /api/chat/status
│       └── static_assets.rs    # serves the embedded web/ via rust-embed
│
├── cmd/smoke/main.rs   # smoke-tests the curriculum via /api/run
├── Dockerfile          # multi-stage build, distroless runtime (~20 MB)
├── .dockerignore
│
└── web/                # frontend (embedded into the binary)
    ├── index.html
    ├── styles/
    │   ├── tokens.css       # design tokens (light + dark theme)
    │   ├── base.css         # resets, page grid
    │   ├── components.css   # shared buttons + icon-btn
    │   ├── sidebar.css
    │   ├── lesson.css
    │   ├── playground.css
    │   └── chat.css
    └── js/                  # ES modules, no build step
        ├── app.js           # composition root: imports + init order
        ├── state.js         # shared state container + localStorage
        ├── dom.js           # $ / $$ query helpers
        ├── api.js           # all fetch() calls live here
        ├── theme.js
        ├── lessons.js       # sidebar + nav + render + progress
        ├── playground.js    # editor + run + output
        ├── chat.js          # chat panel + per-lesson history
        └── markdown.js      # safe md subset for chat replies
```

**Highlights worth opening in your editor:**

- `src/main.rs`: pure wiring. tracing init, dependency construction,
  axum server with graceful shutdown.
- `src/lesson/mod.rs`: the `Repository` trait pattern. Today it's
  `InMemoryRepository`; tomorrow you could add a `FileRepository`
  without changing the api or tutor modules.
- `src/lesson/catalog.rs`: single composition point for lesson order.
  Each `lessons_<category>.rs` exposes a `lessons()` function returning
  a `Vec<Lesson>`; the catalog stitches them in pedagogical order.
- `src/tutor/provider.rs` + `httpx.rs`: the `Provider` trait and the
  one `post_json` helper that all three LLM adapters share. Compare
  the three provider files to see how the protocol-specific
  differences are isolated.
- `src/runner/`: pluggable sandbox. `mod.rs` defines the `Runner`
  trait; `local.rs` shells out to `rustc` (dev default);
  `playground.rs` POSTs to the Rust Playground API (cloud default).
  Pick at startup with `RUNNER=local|playground` or rely on
  auto-detect.
- `src/api/static_assets.rs`: uses `rust-embed` to bake the entire
  `web/` directory into the binary at compile time.
- `web/js/app.js`: the frontend composition root mirrors the backend.

## Build a single binary

```bash
cargo build --release
./target/release/rust-tut
```

Cross-compile for another platform:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

The web assets are baked into the binary via `rust-embed`.

## Code sandbox (Local vs Playground)

The `/api/run` endpoint executes user-submitted Rust code through a
pluggable backend. Pick one with the `RUNNER` environment variable:

| `RUNNER`     | Backend                                  | Use for       | Safety |
| ---          | ---                                      | ---           | ---    |
| `local`      | `rustc` in a temp dir                    | Local dev     | UNSAFE in public |
| `playground` | POST to <https://play.rust-lang.org/execute> | Cloud deploys | Safe — community sandbox |
| unset        | Auto: `playground` if `$K_SERVICE` is set (Cloud Run / Knative), else `local` | Anywhere | Picks the safe default |

**Local backend.** Convenient on a laptop, but it runs untrusted code
with the same UID, filesystem, and network access as the server
process. It's only suitable for `cargo run` on your own machine. Do
not expose a Local-backend instance to the public internet.

**Playground backend.** Sends the source to the community-run Rust
Playground service (the same one that powers
<https://play.rust-lang.org>). User code runs inside a hardened
sandbox with no network, restricted filesystem, and a hard wall-clock
limit. This is the only safe choice for a public-facing deployment.
Override the endpoint with `PLAYGROUND_URL` if you self-host the
playground (the source is at
[rust-lang/rust-playground](https://github.com/rust-lang/rust-playground)).

## Deploy to Google Cloud Run

A multi-stage `Dockerfile` produces a small image that defaults to the
Playground runner, listens on `$PORT`, exposes `/healthz` for probes,
and shuts down gracefully on `SIGTERM`.

One-command deploy (uses Cloud Build under the hood, no local Docker
needed):

```bash
gcloud run deploy rust-tut \
  --source . \
  --region us-central1 \
  --allow-unauthenticated
```

If you want LLM-powered "Ask AI", pass the key as a secret-backed env
var:

```bash
# Store the key in Secret Manager:
echo -n "AIza..." | gcloud secrets create gemini-key --data-file=-

# Reference it on deploy:
gcloud run deploy rust-tut \
  --source . \
  --region us-central1 \
  --allow-unauthenticated \
  --update-secrets=GEMINI_API_KEY=gemini-key:latest
```

The default `RUNNER=playground` is set in the Dockerfile, and the
in-app auto-detection (via `$K_SERVICE`) would pick Playground anyway
if it weren't.

### Manual Docker workflow

```bash
docker build -t rust-tut .
docker run --rm -p 8080:8080 rust-tut
# open http://localhost:8080
```

The image runs as non-root, ignores the bundled `.env` (use
`-e GEMINI_API_KEY=...` instead), and defaults to the Playground
runner.

## Going further

After you finish the lessons, try these projects to cement the language:

1. **JSON CRUD service**: axum + sqlx + Postgres. Add tests with
   `tower::ServiceExt`.
2. **CLI tool**: clap + anyhow + serde. Cross-compile to Linux /
   macOS / Windows.
3. **Async pipeline**: tokio + channels, with `tokio::select!`
   cancellation and graceful shutdown.
4. **WASM module**: compile a small Rust library to WebAssembly with
   `wasm-bindgen` and call it from JS.

Recommended reading:

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (unsafe Rust)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Jon Gjengset's "Crust of Rust"](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) videos.

## License

[MIT](./LICENSE) © 2026 0xm1kr
