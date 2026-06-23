# flamegraph

CPU-heavy prime summation example intended for profiling and flamegraph practice.

## What it shows

- A deterministic hot loop
- A simple target for `perf`, `cargo flamegraph`, or similar tools

## Run

```bash
cargo run --release
```