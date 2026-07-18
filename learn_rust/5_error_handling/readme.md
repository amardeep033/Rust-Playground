# 5 · error handling

Copy one `src/N_*.rs` into `src/main.rs`, then `cargo run`.
Deps (`thiserror`, `anyhow`) are already in `Cargo.toml` — first run downloads them.

## files

| file | topic |
|---|---|
| `1_result_option_question_mark.rs` | `?` on Result AND Option, combinators, `ok_or`/`ok` |
| `2_box_dyn_error.rs` | `Result<T, Box<dyn Error>>` — many error types, one return, no boilerplate |
| `3_thiserror.rs` | typed error enum for libraries, `#[from]` + `?`, match on variants |
| `4_anyhow.rs` | ergonomic propagation for apps, `.context()`, `?` in `main` |

## pointers

| topic | point |
|---|---|
| `?` | early-returns Err/None, else unwraps the value |
| `?` scope | works on **Option too**, not only Result — fn must return the matching type |
| `?` conversion | on Err it calls `.into()` → error must be convertible via `From` |
| thiserror | **libraries**: typed enum variants callers can `match` on |
| `#[from]` | auto-derives `From<SourceErr>` so `?` converts into your enum variant |
| anyhow | **binaries**: one dynamic error type, just surface/display it |
| `.context()` | anyhow: attach a message; `{:#}` prints the full context chain |
| `main` returns | use `-> Result<(), Box<dyn Error>>` (or anyhow `Result<()>`) to use `?` in main |
| library panic? | almost never — return a `Result`; panics are for bugs/broken invariants |
| pick error type | one op → concrete err · many ops/app → `Box<dyn Error>`/anyhow · lib → custom `thiserror` |
| Option combinators | `if let Some(x)`, `map`, `and_then` (chain), `unwrap_or(_else/_default)`, `ok_or` → Result |
| Result↔Option | `res.ok()` drops the error; `opt.ok_or(e)` adds one; `.transpose()` swaps the nesting |

## common questions

**Q: When do I use `Box<dyn Error>` vs `thiserror` vs `anyhow`?**
- `Box<dyn Error>` — quickest, std-only. Many error types → one return via `?`. Caller can't easily match the cause. Good for throwaway/machine-coding.
- `thiserror` — you're writing a **library** and callers need to `match` specific error variants. Typed enum, `#[from]`, `#[error("..")]` = Display.
- `anyhow` — you're writing an **app/binary** and just want to propagate + display errors with `.context()`. Like `Box<dyn Error>` but ergonomic.

**Q: Can the caller recover the concrete error from `Box<dyn Error>`?**
Only by `downcast_ref::<ConcreteError>()` — clumsy. If branching on the cause matters, use a typed enum (thiserror).

**Q: When should code `panic!` instead of returning `Result`?**
Only for **bugs / broken invariants** (unreachable states, failed assertions). Expected failures (bad input, missing file, parse error) → `Result`. Library code should almost never panic.
