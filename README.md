# Rust Playground

Small Rust experiments collected in one repository. Each top-level folder is an independent crate focused on a specific concept, library, or runtime behavior.

## Projects

- `counter` - Actix Web service with Prometheus metrics and a sample payment flow.
- `criterion` - Minimal crate reserved for benchmark experiments.
- `flamegraph` - CPU-bound prime summation example suited for profiling and flamegraphs.
- `graphql` - Async GraphQL + Actix Web example with in-memory book queries and mutations.
- `grpc` - Tonic gRPC client/server example generated from a protobuf definition.
- `kv-store` - Tiny TCP key-value store supporting `SET`, `GET`, `DEL`, and `QUIT`.
- `otel` - OpenTelemetry and logging playground for instrumentation experiments.
- `plots` - Simple plotting examples using Plotters and Textplots.
- `race` - Small Tokio timeout and channel selection example.
- `ratatui` - Terminal UI playground built with Ratatui and Sysinfo.
- `rust-basics` - Runnable examples for ownership, borrowing, lifetimes, `Rc`, `Box`, and strings.
- `rss` - Memory allocation test utility for observing resource usage over time.
- `sorting` - Large in-memory sorting and range-query experiment.
- `webhook-listner` - Basic Actix Web webhook receiver for inspecting incoming requests.

## Quick Start

Run any project from its own folder:

```bash
cd counter
cargo run
```

Use this repo when you want a small, isolated Rust example instead of a single monolithic application.