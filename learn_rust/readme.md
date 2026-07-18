# Rust — 1-day interview revision

Runnable Q&A drills in six ordered folders. Open a folder's `readme.md`, paste any
`src/N_*.rs` into that folder's `src/main.rs`, and `cargo run`. Each file is a
question + snippet + answer (plus cross-questions). **If you can answer every
file's questions, you're ready.** `cheetsheet.md` is the fast final glance.

## 1 · [ownership & borrowing](1_ownership/readme.md)
| file | you should be able to answer |
|---|---|
| 1_move_vs_copy | move vs copy, Copy types, NLL, what ownership prevents |
| 2_borrowing | `&T`/`&mut T` rules, why one `&mut`, compile-time check |
| 3_dangling_and_returns | dangling vs use-after-free, return owned not `&local` |

## 2 · [essentials](2_essentials/readme.md) — collections, strings, iterators, closures, I/O
| file | you should be able to answer |
|---|---|
| 1_scalars_tuples_arrays | integer overflow, tuple vs array vs slice, `usize`/`char` |
| 2_strings | `split_once`/`trim`/`parse`, why no `s[i]` |
| 3_vec | iter vs iter_mut vs into_iter, `with_capacity`, `get` vs `[]` |
| 4_hashmap | `entry().or_insert()` freq count, ordering, `get` |
| 5_iterators | lazy, `filter_map`, what drives it, `fold` vs `reduce` |
| 6_closures | Fn/FnMut/FnOnce, why `move`, returning closures |
| 7_sorting_and_heap | `sort_by`, top-K min-heap, merge k sorted |
| 8_file_stream_batch | `BufReader` streaming, batching, why `flush` |

## 3 · [lifetimes](3_lifetimes/readme.md)
| file | you should be able to answer |
|---|---|
| 1_why_lifetimes | why `longest` fails, what `'a` means, do they extend data |
| 2_struct_lifetime | why a struct with `&str` needs `<'a>`, how to avoid it |
| 3_elision | the 3 elision rules, why 2 inputs don't elide, `'static` |

## 4 · [traits, generics, box & dispatch](4_traits_generics/readme.md)
| file | you should be able to answer |
|---|---|
| 1_str_types_and_dst | str/&str/String, the 3 DSTs, `&str` param, layouts |
| 2_traits | no inheritance, default methods, orphan rule |
| 3_common_traits | Debug vs Display, derivable traits, From/Into, `?` conversion |
| 4_generics_and_dispatch | static vs dynamic dispatch, monomorphization, vtable |
| 5_generics_deep | bounded impl, `where`, blanket impl, supertrait |
| 6_box_and_trait_objects | why `Vec<Box<dyn>>` not `Vec<dyn>`, object safety, recursion |

## 5 · [error handling](5_error_handling/readme.md)
| file | you should be able to answer |
|---|---|
| 1_result_option_question_mark | `?` on Option too, desugar, combinators |
| 2_box_dyn_error | `Box<dyn Error>` for many error types, downcast, vs the others |
| 3_thiserror | typed errors to `match` on, `#[from]` |
| 4_anyhow | anyhow vs thiserror, `.context()`, `?` in `main`, when to panic |

## 6 · [concurrency & async](6_concurrency/readme.md)
| file | you should be able to answer |
|---|---|
| 1_1..1_4 std | threads+Arc, RwLock, Mutex, mpsc (`drop(tx)`) |
| 2_1..2_4 tokio | spawn/await, async RwLock/Mutex, bounded mpsc |
| 3_1 rc_refcell · 3_2 arc_send_sync | single-thread shared+mutable; Send/Sync, data race vs race condition |
| 4_1 async_await · 4_2 join_select_backpressure | lazy futures, blocking trap, `join!`/`select!`, backpressure |

---

Reference (root): [`cheetsheet.md`](cheetsheet.md) — all-tables Rust quick-glance.
