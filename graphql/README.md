# graphql

Async GraphQL playground built with Actix Web and in-memory book storage.

## What it shows

- GraphQL queries and mutations in Rust
- An in-memory schema with `Book`, `QueryRoot`, and `MutationRoot`
- Playground UI and GraphQL HTTP endpoint

## Run

```bash
cargo run
```

Default routes:

- `GET /playground`
- `POST /graphql`