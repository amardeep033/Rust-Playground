# 8 - random Rust notes

Small standalone Rust questions. Copy one `src/N_*.rs` into `src/main.rs`, then
`cargo run`. Files are study notes, so they are not wired into the binary on
their own.

## files

| file | topic |
|---|---|
| `1_stream_json.rs` | streaming JSON with `serde_json::Deserializer` |
| `2_eq_partial_eq_ord_partial_ord.rs` | equality and ordering traits |
| `3_serde_vs_serde_json.rs` | `serde` data model vs JSON format support |

## quick pointers

| topic | point |
|---|---|
| JSON array | normal JSON is often one big value; parse it as `Vec<T>` or stream its elements |
| NDJSON | newline-delimited JSON is naturally streamable: one JSON value per line |
| `Deserializer::from_reader` | parses from any `Read`; useful for files, stdin, network bodies |
| `into_iter::<T>()` | reads value after value from a stream, instead of one whole JSON document |
| `PartialEq` | `==` and `!=`; comparison can exist for only some meaning of equality |
| `Eq` | marker trait saying equality is reflexive: every value equals itself |
| `PartialOrd` | `<`, `<=`, `>`, `>=`; comparison can return "not comparable" |
| `Ord` | total ordering; every pair has a stable order |
| floats | `f32` and `f64` are `PartialEq`/`PartialOrd`, not `Eq`/`Ord`, because `NaN != NaN` |
| `serde` | traits and data model: `Serialize`, `Deserialize` |
| `serde_json` | one concrete format crate for JSON strings, values, readers, writers |
| why both? | `serde_json` uses serde's traits; your structs derive serde traits, then JSON/TOML/etc. formats plug into them |

## quick chooser

| need | reach for |
|---|---|
| read one JSON document | `serde_json::from_str` or `from_reader` |
| read many adjacent JSON values | `serde_json::Deserializer::from_reader(...).into_iter()` |
| read one JSON object per line | loop over lines, then `serde_json::from_str` per line |
| model data for many formats | derive `serde::Serialize` and `serde::Deserialize` |
| emit or parse JSON specifically | use `serde_json` |
| wonder why not only `serde_json` | because format crates should not own your data traits; serde lets one derive work with many formats |
| sort with no weird edge cases | implement/derive `Ord` |
| compare values that may be unordered | use `PartialOrd` |
| use a type as `BTreeMap` key | it needs `Ord` |
| use a type as `HashMap` key | it needs `Eq + Hash` |
