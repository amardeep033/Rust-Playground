# 2 · essentials (collections, strings, iterators, closures, I/O)

The catch-all bucket of everyday practical Rust. Copy one `src/N_*.rs` into
`src/main.rs`, then `cargo run`.

## files

| file | topic |
|---|---|
| `1_scalars_tuples_arrays.rs` | integer overflow, tuples, arrays, slices |
| `2_strings.rs` | `split_once`/`trim`/`parse` — line parsing |
| `3_vec.rs` | `Vec` ops + iter vs iter_mut vs into_iter (ownership) |
| `4_hashmap.rs` | `entry().or_insert()` frequency count, `and_modify` |
| `5_iterators.rs` | lazy adapters: map/filter/filter_map/fold/zip/take/skip |
| `6_closures.rs` | Fn / FnMut / FnOnce capture, `move` closures |
| `7_sorting_and_heap.rs` | `sort_by`/`sort_by_key`, `BinaryHeap` top-K & merge |
| `8_file_stream_batch.rs` | `BufReader`/`BufWriter`, stream `lines()`, batching |

## pointers

| topic | point |
|---|---|
| overflow | debug PANICS, release WRAPS → `wrapping_`/`checked_`/`saturating_add` |
| tuple/array/slice | tuple = fixed mixed types; array `[T;N]` = fixed one type on stack; slice `&[T]` = borrowed view (DST) |
| iter form | `for x in v` moves (`into_iter`), `&v` borrows (`iter`), `&mut v` mutates (`iter_mut`) |
| lazy | adapters do nothing until a consumer (`collect`/`sum`/`for`) runs |
| filter_map | transform + drop invalid (`.ok()`) in one pass — skip bad rows |
| entry API | `entry(k).or_insert(0)` = insert-or-get in one lookup (freq count) |
| dedup | only removes *consecutive* dups → `sort()` first |
| get | `HashMap::get` / `Vec::get` return `Option`; `v[i]` panics OOB |
| closures | capture by weakest access: Fn (`&`), FnMut (`&mut`), FnOnce (owns); `move` = by value |
| move closures | required when a closure outlives the scope (`thread::spawn`, `tokio::spawn`) |
| window/chunk | `slice.windows(n)` (overlapping) / `.chunks(n)` (non-overlapping) for sliding scans |
| sort | `sort_by_key(key)`; `sort_by(|a,b| b.cmp(a))` descending; `sort_unstable*` is faster |
| top-K | size-k `BinaryHeap` (max-heap); wrap in `std::cmp::Reverse` for a min-heap → O(n log k) |
| BufReader | `lines()` streams (constant memory); wrap file to avoid a syscall per line; `flush` BufWriter |
| transpose | `Vec<Result<T,E>>` → `Result<Vec<T>,E>` via `.collect()`; `Option<Result>` ↔ `.transpose()` |
