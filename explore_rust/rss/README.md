# rss

Memory allocation test utility for observing process memory growth over time.

## What it shows

- Stepwise heap allocation in MB-sized chunks
- A simple long-running process for system monitoring experiments

## Run

```bash
cargo run -- <mb_per_step> <hold_seconds>
```