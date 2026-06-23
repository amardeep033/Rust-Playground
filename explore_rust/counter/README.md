# counter

Actix Web sample service that exposes a fake payment endpoint and Prometheus metrics.

## What it shows

- HTTP routes with Actix Web
- Counters and latency histograms with Prometheus
- A simple metrics endpoint for scraping

## Run

```bash
cargo run
```

Default routes:

- `GET /initiate-payment`
- `GET /metrics`