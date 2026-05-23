# race

Small Tokio example that races a channel receive against a timeout using `tokio::select!`.

## What it shows

- Async task spawning
- `mpsc` channels
- Timeout behavior with `select!`

## Run

```bash
cargo run
```