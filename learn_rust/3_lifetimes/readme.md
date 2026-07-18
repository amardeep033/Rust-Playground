# 3 · lifetimes

Copy one `src/N_*.rs` into `src/main.rs`, then `cargo run`.

## files

| file | topic |
|---|---|
| `1_why_lifetimes.rs` | `longest<'a>` — tie output ref to inputs; the "outlives" trap |
| `2_struct_lifetime.rs` | struct holding `&str` needs `<'a>`; can't outlive borrowed data |
| `3_elision.rs` | the 3 elision rules; when you don't need to write `'a` |

## pointers

| topic | point |
|---|---|
| what they are | annotations naming *how long a ref is valid* — they don't extend data's life |
| core rule | no reference may outlive the data it points to |
| when needed | compiler can't infer which input an output ref ties to (2+ input refs) |
| struct ref | any struct holding a `&` needs a lifetime param; instance can't outlive the ref |
| elision 1 | each `&` param gets its own lifetime |
| elision 2 | exactly one input ref → output gets that same lifetime |
| elision 3 | `&self`/`&mut self` present → its lifetime is used for all outputs |
| owned escape | return `String` (owned) instead of `&str` → no lifetime needed |
| `'static` | lives for the whole program (string literals); not a fix-all for errors |
