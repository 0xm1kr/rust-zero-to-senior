# syntax=docker/dockerfile:1.7

# ─── Stage 1: build a static binary ─────────────────────────────────────────
# Pin a Rust toolchain for reproducible builds. Bump it deliberately.
FROM rust:1.86-bookworm AS build

WORKDIR /src

# Prime the Cargo cache as its own layer so dependency changes don't bust
# the source-only rebuild path.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src cmd/smoke \
    && echo 'fn main() {}' > src/main.rs \
    && echo 'fn main() {}' > cmd/smoke/main.rs \
    && cargo build --release \
    && rm -rf src cmd/smoke target/release/rust-tut target/release/smoke

# Copy the rest of the source.
COPY . .

# Touch sources so cargo notices and rebuilds.
RUN touch src/main.rs cmd/smoke/main.rs \
    && cargo build --release --bin rust-tut \
    && cp target/release/rust-tut /out-rust-tut

# ─── Stage 2: minimal runtime ───────────────────────────────────────────────
# distroless/cc gives us a tiny image with the libc/libssl symbols rustls
# doesn't need but that some indirect deps still link. We pin nonroot.
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

COPY --from=build /out-rust-tut /rust-tut

EXPOSE 8080

# Default to the Playground runner so the container is safe by default.
ENV RUNNER=playground
ENV RUST_LOG=info

USER nonroot:nonroot
ENTRYPOINT ["/rust-tut"]
