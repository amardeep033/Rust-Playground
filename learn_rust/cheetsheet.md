# Rust cheatsheet — final glance

All-tables reference distilled from the notes. The six folders hold the runnable
Q&A depth; this is the last-minute scan. Jump to a section; if a row surprises
you, go drill that folder.

## ownership & borrowing

| rule | detail |
|---|---|
| owner | every value has exactly 1 owner; dropped when the owner leaves scope |
| move | non-Copy assign transfers ownership → original invalid (`String`,`Vec`,`Box`) |
| copy | Copy assign duplicates bits → both valid (`i32`,`bool`,`char`,`f64`,`&T`, tuples of Copy) |
| aliasing | many `&T` **OR** one `&mut T`, never both at once |
| NLL | a borrow ends at its **last use**, not at `}` |
| mutate | needs a `mut` binding **and** a `&mut` borrow |
| dangling | can't return `&` to a local (it drops) → return owned `T` |
| checker | runs at **compile time**; `RefCell` moves the same check to **runtime** |
| prevents | use-after-free, double-free, dangling, data races |

## types

| topic | detail |
|---|---|
| `str` | bare UTF-8, a **DST** (size unknown) → only behind a pointer |
| `&str` | fat pointer `(ptr,len)`, borrowed, immutable; **prefer as param** (accepts `&String`, literals) |
| `String` | `(ptr,len,cap)`, heap-owned, growable |
| DSTs (3) | `str`, `[T]`, `dyn Trait` — always behind `&`/`Box`/`Rc` |
| overflow | debug **panics**, release **wraps** → `wrapping_`/`checked_`/`saturating_add` |
| `usize` | pointer-sized int for indexing/lengths |
| `char` | 4 bytes, a Unicode scalar (not a byte); can't index `String` by `[i]` |
| tuple/array/slice | tuple = fixed mixed types; `[T;N]` = fixed one type on stack; `&[T]` = borrowed view |

## smart pointers & Send/Sync

| type | ownership | thread-safe | mutability |
|---|---|---|---|
| `Box<T>` | single heap owner | if `T: Send` | normal borrow rules |
| `Rc<T>` | shared, non-atomic count | no | read-only |
| `Arc<T>` | shared, atomic count | yes | read-only |
| `RefCell<T>` | single | no | interior mut, **runtime**-checked (panics) |
| `Rc<RefCell<T>>` | shared | no | shared + mutable, 1 thread |
| `Arc<Mutex<T>>` | shared | yes | exclusive lock |
| `Arc<RwLock<T>>` | shared | yes | many readers OR one writer |

| marker | meaning |
|---|---|
| `Send` | ownership of `T` can be **moved** to another thread |
| `Sync` | `&T` can be **shared** across threads |
| `Rc` | neither (non-atomic refcount) |
| `Arc` | both, when `T` is |
| `RefCell` | Send (if T:Send), **not Sync** |

## traits, generics, dispatch

| topic | detail |
|---|---|
| trait | a contract of methods; no inheritance — use traits + composition |
| default method | trait supplies a body; impls inherit or override |
| `impl Trait` param | sugar for `<T: Trait>`; use explicit generic to name `T` |
| orphan rule | trait OR type must be local to impl it |
| static dispatch | generics/`impl Trait` → monomorphized copy per type, zero cost, one type per call |
| dynamic dispatch | `dyn Trait` → runtime vtable lookup, allows mixed types in one `Vec` |
| `dyn Trait` layout | fat pointer `(data*, vtable*)` |
| `impl Trait` return | one hidden concrete type; two types → `Box<dyn Trait>` |
| object safety | no `-> Self`, no generic methods in a `dyn` trait → no `dyn Clone` |
| Box uses | trait objects, recursive types (known size), large values |
| `Vec<Box<dyn>>` | elements need equal size; Box = same-size fat pointer each |
| Debug vs Display | `Debug` (`{:?}`) derivable; `Display` (`{}`) hand-written |
| derivable | Debug, Clone, PartialEq/Eq, Hash, Default, PartialOrd/Ord |
| From/Into | impl `From<T>` → get `Into` free; `?` uses `From` to convert errors |
| blanket impl | `impl<T: Bound> Trait for T` (e.g. `ToString` for all `Display`) |
| supertrait | `trait A: B` — `A` implementors must also impl `B` |

## lifetimes

| rule | detail |
|---|---|
| meaning | annotations that **name** how long refs are valid; don't extend data's life |
| core | no reference may outlive the data it points to |
| when needed | 2+ input refs and the compiler can't tell which the output ties to |
| struct ref | a struct holding `&` needs `<'a>`; can't outlive the borrowed data |
| elision 1 | each `&param` gets its own lifetime |
| elision 2 | one input lifetime → used for all outputs |
| elision 3 | `&self`/`&mut self` present → its lifetime → all outputs |
| `'static` | lives whole program (string literals); not a fix-all |

## error handling

| topic | detail |
|---|---|
| `?` | early-returns Err/None else unwraps; works on **Option too** |
| `?` desugar | `Err(e) => return Err(e.into())` — converts via `From` |
| `main` + `?` | `fn main() -> Result<(), Box<dyn Error>>` / `anyhow::Result<()>` |
| **thiserror** | libraries: typed enum, `#[error("..")]`=Display, `#[from]` for `?`, callers `match` |
| **anyhow** | apps: one dynamic error, `.context("..")`, `{:#}` prints chain |
| panic? | library code almost never — return `Result` |

| Option/Result move | API |
|---|---|
| value or default | `unwrap_or(_else / _default)` |
| transform | `map` · error: `map_err` |
| chain | `and_then` |
| Option→Result | `ok_or(e)` |
| Result→Option | `.ok()` (drops error) |
| swap nesting | `.transpose()` (`Option<Result>` ↔ `Result<Option>`) |
| collect | `Vec<Result<T,E>>` → `Result<Vec<T>,E>` via `.collect()` |

| error type choice | use |
|---|---|
| one operation | concrete error (`std::num::ParseIntError`) |
| many ops / app | `Box<dyn Error>` or `anyhow` |
| library, matchable | custom enum + `thiserror` |

## API by task

| strings | iterators | Vec | HashMap |
|---|---|---|---|
| `split_once` cut k/v | `map` transform | `push`/`pop`/`remove` | `entry().or_insert()` count |
| `trim` clean | `filter` keep | `sort`/`dedup`/`retain` | `and_modify().or_insert()` |
| `parse::<T>()`→Result | `filter_map` drop-invalid | `get(i)` safe / `[i]` panics | `get`→Option, `remove` |
| `starts_with`/`replace` | `fold`/`reduce`/`collect` | `iter`/`iter_mut`/`into_iter` | `keys`/`values`/`iter` |
| `chars`/`bytes` | `zip`/`enumerate`/`take`/`skip` | `with_capacity(n)` | BTreeMap = sorted keys |

| File I/O | async / tokio |
|---|---|
| `BufReader::lines()` stream (const mem) | `tokio::spawn` (Future+Send+'static) |
| `read_to_string` (whole file) | `join!` all · `select!` first (cancels rest) |
| `BufWriter` + `flush` | `mpsc` bounded = backpressure |
| `File::open`/`create`, `write_all` | `spawn_blocking` for CPU-heavy |

## sorting / top-K

| want | how |
|---|---|
| ascending by key | `sort_by_key(\|x\| key)` |
| descending | `sort_by(\|a,b\| b.cmp(a))` |
| faster, no stability | `sort_unstable*` |
| sliding window | `slice.windows(n)` overlap · `.chunks(n)` non-overlap |
| top-K largest | size-k `BinaryHeap` (max-heap) + `std::cmp::Reverse` = min-heap → O(n log k) |
| merge k sorted | heap of stream heads: pop smallest, push that stream's next |

## concurrency & async

| topic | detail |
|---|---|
| thread vs task | OS thread = heavy/blocking work; tokio task = lightweight, many I/O jobs |
| move closure | required for spawn — closure may outlive the frame, must own captures |
| join/await | waits for finish; does NOT start (already running since spawn) |
| Mutex | one holder, cheaper; writes frequent |
| RwLock | many readers OR one writer, tracks reader count (more overhead); reads ≫ writes |
| mpsc | multi-producer/single-consumer; loop ends when **all** senders drop → `drop(tx)` |
| future | lazy state machine; nothing runs until polled (`.await`/`spawn`); async ≠ new thread |
| `.await` | suspends the task, yields the thread to the executor |
| blocking trap | `std::thread::sleep`/CPU in async freezes the executor → `spawn_blocking` |
| async lock | tokio Mutex/RwLock only needed if held across `.await` |
| backpressure | bounded channel: `send().await` blocks when full → slows producer to consumer |

| memory-safety term | what | Rust prevents? |
|---|---|---|
| use-after-free | dereference freed memory | ✅ ownership+lifetimes |
| double free | free twice | ✅ single ownership |
| dangling pointer | invalid ref *exists* (state) | ✅ lifetimes |
| **data race** | unsync concurrent access, ≥1 write (memory-safety bug) | ✅ Send/Sync + aliasing |
| lock contention | threads wait on a busy lock (perf) | ❌ |
| deadlock | circular lock wait (never resolves) | ❌ |
| **race condition** | timing/order-dependent logic bug (broader) | ❌ |

> All data races are race conditions; not all race conditions are data races.

## common misconceptions (say the right thing)

| you might say | correct |
|---|---|
| "borrow the ownership" | you borrow the **value** (`&T`/`&mut T`) or **move** ownership |
| "String not Copy because heap" | it **owns** heap; deep-copy is costly → explicit `.clone()` |
| "Box because type is dynamic" | Box = single heap ownership: recursion, trait objects, big values |
| "Rc lacks Sync" | Rc is **neither Send nor Sync** (non-atomic refcount) |
| "GC frees after last use" | freed when the **owner drops** (RAII, deterministic) |
| "call `obj.drop()`" | use `std::mem::drop(obj)`; `Drop::drop` runs automatically |
| "Mutex lets all threads read" | Mutex = **exclusive** access; Arc = shared ownership |
| "OS thread is lightweight" | OS threads are **heavy** (kernel-scheduled, own stack) |
| "future runs in background" | futures are **lazy**, run only when polled |
| "two owners update = race condition" | that's a **data race**; race condition = timing-dependent |

## compile-error checklist (scan any snippet)

| # | smell | result |
|---|---|---|
| 1 | non-Copy used after move | E0382 |
| 2 | borrow active during move / 2nd borrow | E0505 / E0502 / E0499 |
| 3 | `&T` and `&mut T` overlap | borrow conflict |
| 4 | return `&` to a local | E0515 (return owned) |
| 5 | `{}`/`.clone()` on unbounded `T` | E0277 (add bound) |
| 6 | `Rc` across threads | not Send |
| 7 | `dyn Trait` not behind `&`/`Box` | DST error |
| 8 | `RefCell` double-borrow | compiles, **panics at runtime** |
| 9 | `?` in a `()`-returning fn | needs Result/Option |
| 10 | integer overflow | panics (debug) / wraps (release) |
| 11 | blocking call in async | no error, **executor starvation** |

## advanced (know it exists)

| topic | one-liner |
|---|---|
| `Deref` coercion | why `&String` works where `&str` is expected (auto `&String`→`&str`) |
| `Pin`/`Unpin` | pinning fixes a future's address; needed for self-referential async state machines |
| self-referential struct | async futures can hold refs into their own data → must be pinned |
| impl vs dyn perf | `impl Trait`/generics = static (fast); `dyn` = one vtable indirection |
| `CoerceUnsized` | the mechanism behind DST coercions (`Box<T>`→`Box<dyn Trait>`) |
| JoinHandle | `std::thread::JoinHandle.join()` blocks the thread; `tokio::task::JoinHandle.await` yields the task |

## machine-coding approach

| step | do |
|---|---|
| clarify first | duplicates? sorted? invalid-input? complexity? memory vs speed? input size? |
| idiomatic | iterators over loops · `&str` over `&String` · avoid needless `.clone()` · `Result`/`?` over `.unwrap()` |
| AI/docs allowed | use as reference ("signature of `HashMap::entry`?"), not to solve the problem |
