# Rust Playground

Personal Rust workspace, split into three areas by purpose: structured interview
revision, timed machine-coding practice, and free-form library/runtime exploration.
Each leaf folder is an independent crate.

## Layout

```
.
├── learn_rust/     # ordered interview revision drills + cheatsheet
├── machine_code/   # timed machine-coding problems (easy → hard)
└── explore_rust/   # independent crates exploring specific libraries/runtime behavior
```

### [`learn_rust/`](learn_rust/readme.md) — interview revision

Six ordered folders of runnable Q&A drills. Paste a `src/N_*.rs` file into that
folder's `src/main.rs` and `cargo run`.
[`cheetsheet.md`](learn_rust/cheetsheet.md) is the all-tables last-minute glance.

- [`1_ownership`](learn_rust/1_ownership/readme.md) - move vs copy, borrowing rules, dangling references.
- [`2_essentials`](learn_rust/2_essentials/readme.md) - collections, strings, iterators, closures, file I/O.
- [`3_lifetimes`](learn_rust/3_lifetimes/readme.md) - why lifetimes exist, struct lifetimes, elision rules.
- [`4_traits_generics`](learn_rust/4_traits_generics/readme.md) - traits, generics, static vs dynamic dispatch, `Box`/trait objects.
- [`5_error_handling`](learn_rust/5_error_handling/readme.md) - `Result`/`Option`/`?`, `Box<dyn Error>`, `thiserror`, `anyhow`.
- [`6_concurrency`](learn_rust/6_concurrency/readme.md) - threads, tokio, `Rc`/`RefCell`, `Send`/`Sync`, async/await, backpressure.

### [`machine_code/`](machine_code/readme.md) — timed practice

Machine-coding problems, easy to hard. Rules: 30 min/question, no AI
assistance, docs/search only. Each folder has a `question.md`.

- [`0_log_ip_top_k`](machine_code/0_log_ip_top_k) - parsing, `?` error propagation, top-k heap.
- [`1_log_error_count_by_service`](machine_code/1_log_error_count_by_service) - basic iteration + HashMap counting.
- [`2_csv_group_by_agg`](machine_code/2_csv_group_by_agg) - file/CSV parsing + group-by-count.
- [`3_merge_sorted_log`](machine_code/3_merge_sorted_log) - merge-sorted-streams algorithm.
- [`4_static_vs_dynamic_dispatch`](machine_code/4_static_vs_dynamic_dispatch) - static vs dynamic dispatch, trait objects.
- [`5_json_parse_error_types`](machine_code/5_json_parse_error_types) - JSON parsing + custom error types.
- [`6_concurrent_service_counter`](machine_code/6_concurrent_service_counter) - shared mutable state across async tasks.
- [`7_async_pipeline_channels`](machine_code/7_async_pipeline_channels) - multi-stage async pipeline via channels.
- [`8_book_scraper_server`](machine_code/8_book_scraper_server) - actix-web service + live scraping (reqwest).

### [`explore_rust/`](explore_rust/) — library & runtime exploration

Unstructured playground; each folder is an independent crate for one concept:

- [`counter`](explore_rust/counter) - Actix Web service with Prometheus metrics and a sample payment flow.
- [`criterion`](explore_rust/criterion) - Minimal crate reserved for benchmark experiments.
- [`flamegraph`](explore_rust/flamegraph) - CPU-bound prime summation example suited for profiling and flamegraphs.
- [`graphql`](explore_rust/graphql) - Async GraphQL + Actix Web example with in-memory book queries and mutations.
- [`grpc`](explore_rust/grpc) - Tonic gRPC client/server example generated from a protobuf definition.
- [`kv-store`](explore_rust/kv-store) - Tiny TCP key-value store supporting `SET`, `GET`, `DEL`, and `QUIT`.
- [`plots`](explore_rust/plots) - Simple plotting examples using Plotters and Textplots.
- [`race`](explore_rust/race) - Small Tokio timeout and channel selection example.
- [`ratatui`](explore_rust/ratatui) - Terminal UI playground built with Ratatui and Sysinfo.
- [`rss`](explore_rust/rss) - Memory allocation test utility for observing resource usage over time.
- [`sorting`](explore_rust/sorting) - Large in-memory sorting and range-query experiment.
- [`webhook-listner`](explore_rust/webhook-listner) - Basic Actix Web webhook receiver for inspecting incoming requests.

## Quick Start

Run any project from its own folder:

```bash
cd explore_rust/counter
cargo run
```

Use this repo when you want a small, isolated Rust example instead of a single monolithic application.
