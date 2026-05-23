# kv-store

Tiny TCP key-value store implemented with Tokio and a shared in-memory `HashMap`.

## Supported commands

- `SET key value`
- `GET key`
- `DEL key`
- `QUIT`

## Run

```bash
cargo run
```

The server listens on `127.0.0.1:6379`.