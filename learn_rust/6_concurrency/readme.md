# 6 · concurrency & async

Copy one `src/N_*.rs` into `src/main.rs`, then `cargo run`.
`tokio` is already in `Cargo.toml` (the `2_*` and `4_*` files need it) — first run
downloads it.

## files

Reading order builds up: shared ownership → std locks → tokio locks → async.

| file | topic |
|---|---|
| `3_1_rc_refcell.rs` | `Rc` shared owner + `RefCell` interior mutability (single thread) |
| `3_2_arc_send_sync.rs` | `Arc<Mutex>` counter across threads, `move`, Send/Sync, data race |
| `1_1_std_read.rs` | `Arc` read-only sharing across `std::thread` |
| `1_2_std_rwlock.rs` | `std::sync::RwLock` |
| `1_3_std_mutex.rs` | `std::sync::Mutex` |
| `1_4_std_mpsc.rs` | `std::sync::mpsc`, multi-producer / single-consumer |
| `2_1_tokio_read.rs` | `Arc` read-only sharing across `tokio::spawn` tasks |
| `2_2_tokio_rwlock.rs` | `tokio::sync::RwLock` (async lock) |
| `2_3_tokio_mutex.rs` | `tokio::sync::Mutex` (async lock) |
| `2_4_tokio_mpsc.rs` | `tokio::sync::mpsc` (bounded, async) |
| `4_1_async_await.rs` | `async fn` = lazy future, `.await`, blocking trap, `spawn_blocking` |
| `4_2_join_select_backpressure.rs` | `join!` (all) vs `select!` (first), bounded-channel backpressure |

## pointers

| topic | point |
|---|---|
| thread vs task | `thread` = OS thread, heavy/blocking. tokio task = lightweight, many light/IO |
| Rc vs Arc | `Rc` non-atomic count, single thread. `Arc` atomic count, cross-thread |
| RefCell | interior mutability, borrow rules checked at RUNTIME → can panic (not compile) |
| Rc+RefCell | single-thread shared+mutable. Arc+Mutex = the multi-thread version |
| Send / Sync | Send = move `T` to another thread; Sync = share `&T`. `Rc` is neither |
| move | `move` closure transfers captured values into the thread/task |
| RwLock | many readers OR one writer, tracks reader count → more overhead. reads >> writes |
| Mutex | one holder period, cheaper. writes frequent |
| join/await | wait for finish; does NOT start it (already running since spawn) |
| handle+unwrap | handle = ref to thread/task; `unwrap()` panics if the spawned code panicked |
| tokio start | task runs in background right after `spawn`; `.await` only waits on the result |
| future | lazy: nothing runs until polled (`.await`/`tokio::spawn`). async ≠ new thread |
| blocking trap | `std::thread::sleep` / CPU work in async freezes the executor → `spawn_blocking` |
| data race | unsynced access + ≥1 write, same memory — a *memory-safety* bug; safe Rust prevents it |
| race condition | timing/ordering-dependent *logic* bug; broader — can happen even with locks (Rust does NOT prevent) |
| data race ⊂ race condition | all data races are race conditions, but not all race conditions are data races |
| Rust prevents | UAF, double-free, dangling, data races. NOT: deadlock, lock contention, logical race conditions |
| contention/deadlock | contention = waiting on a busy lock (resolves). deadlock = circular wait (never) |
| join! vs select! | `join!` = all complete; `select!` = first wins, rest cancelled |
| backpressure | bounded channel: `send().await` blocks when full → slows producers to match consumer |
| mpsc gotcha | receiver loop ends only when ALL senders drop; forgetting `drop(tx)` blocks forever |

## the confusing words (blocking / sync / async / await / handle)

| term | what it means |
|---|---|
| blocking | the OS thread stops and does nothing until the call returns (`std` Mutex lock, `thread::sleep`, file read) |
| non-blocking | the call returns immediately or **yields** instead of parking the thread |
| sync | runs to completion inline; the caller waits right there |
| async | an `async fn` returns a **lazy Future**; marks code that can suspend at `.await` points — it is NOT a thread |
| `.await` | poll a future; if not ready, **suspend this task** and hand the thread to the executor, resume later |
| handle | a reference to a spawned thread/task (`JoinHandle`); `join()`/`.await` it to wait for and read its result |

## common questions

**Q: When a real thread vs a tokio task?**
OS thread (`std::thread`) for **heavy/blocking CPU work** — a real kernel thread. Tokio task (`tokio::spawn`) for **many concurrent I/O-bound jobs** — lightweight, multiplexed onto a small worker pool. CPU-heavy work inside async → `spawn_blocking`.

**Q: When is a type Send-only, Sync-only, both, or neither?**

| | meaning | example |
|---|---|---|
| Send only (not Sync) | can MOVE to another thread, but can't share `&T` | `Cell`/`RefCell`, `mpsc::Receiver` |
| Sync only (not Send) | `&T` shareable, but the value is tied to its thread | `MutexGuard` (must unlock on the same thread) |
| both | move and share freely | `Arc<T>` (T: Send+Sync), plain data |
| neither | can't cross threads at all | `Rc`, raw pointers `*mut T` |

**Q: Mutex vs RwLock vs atomic — when each?**
Atomic (`AtomicUsize`, …) for a single primitive counter/flag (lock-free). Mutex for one-writer-at-a-time on any data. RwLock only when reads massively outnumber writes (it costs more to track readers).

**Q: `join!` vs `spawn`?**
`join!` runs futures concurrently **on the current task** and waits for all. `spawn` hands a task to the runtime to run **independently** (must be `Send + 'static`).
