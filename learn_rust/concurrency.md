# Concurrency and Async

## OS Threads

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("running in thread");
    });
    handle.join().unwrap(); // block until thread finishes
}
```

### Capturing data: must `move`

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    let h = thread::spawn(move || { // move takes ownership
        println!("{data:?}");
    });
    h.join().unwrap();
}
```

### What happens without `move`?
```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    let h = thread::spawn(|| println!("{data:?}")); // compile error
    h.join().unwrap();
}
```
**Compile error**: closure may outlive the current function, but it borrows `data` which is owned by the current function.

Thread may outlive the stack frame where `data` lives. `move` transfers ownership to the thread.

---

## Message Passing (Channels)

Send data between threads without shared state.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello from thread").unwrap();
    });

    let msg = rx.recv().unwrap(); // blocks until message arrives
    println!("{msg}");
}
```

`mpsc` = **m**ultiple **p**roducer, **s**ingle **c**onsumer. Clone `tx` for multiple senders:

```rust
let tx2 = tx.clone();
thread::spawn(move || tx2.send("from thread 2").unwrap());
thread::spawn(move || tx.send("from thread 1").unwrap());

for msg in rx { // iterate until all senders dropped
    println!("{msg}");
}
```

---

## Shared Mutable State

See [smart_pointers.md] for full detail. Quick pattern:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("{}", counter.lock().unwrap()); // 5
}
```

---

## Async / Await

### Core model
- `async fn` returns a `Future<Output = T>` — lazy, doesn't run until polled.
- `.await` suspends the current task and yields to the executor.
- Executor (Tokio, async-std) drives all futures by calling `poll()`.

```rust
async fn fetch() -> String {
    "data".to_string()
}

#[tokio::main]
async fn main() {
    let result = fetch().await;
    println!("{result}");
}
```

### What is a Future?
A value representing computation not yet complete. The executor calls `poll()` repeatedly.
When a future can't make progress (e.g., waiting for I/O), it returns `Poll::Pending` and the executor runs other tasks. When done, returns `Poll::Ready(value)`.

---

## Blocking in Async: The Critical Trap

### What happens?
```rust
#[tokio::main]
async fn main() {
    process().await;
}

async fn process() -> String {
    std::thread::sleep(std::time::Duration::from_secs(2)); // WRONG
    "done".to_string()
}
```
**Compiles fine. But this is broken.**

`std::thread::sleep` blocks the OS thread. Tokio's executor runs on a thread pool (by default 1 thread per CPU core). If you block one thread, every other task scheduled on that thread is frozen for 2 seconds.

**Fix**: use async-aware sleep
```rust
async fn process() -> String {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    "done".to_string()
}
```

**Rule**: never use blocking APIs inside async functions. The executor thread must remain free to poll other futures.

---

## Running Tasks Concurrently

### Sequential (wrong for independent tasks)
```rust
let a = task_a().await;
let b = task_b().await;
// total time = a + b
```

### Concurrent (correct)
```rust
use tokio::join;

let (a, b) = join!(task_a(), task_b()); // both run concurrently
// total time ≈ max(a, b)
```

`join!`: wait for **all** futures to complete.  
`select!`: wait for the **first** to complete, cancel the rest.

```rust
use tokio::select;

select! {
    result = fetch_from_primary()   => handle(result),
    result = fetch_from_fallback()  => handle(result),
}
// Whichever resolves first wins; the other is dropped
```

---

## `tokio::spawn` for Background Tasks

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // runs concurrently with main
        heavy_async_work().await
    });

    do_other_stuff().await;

    let result = handle.await.unwrap(); // wait for spawned task
}
```

### Send constraint — what happens?
```rust
async fn bad() {
    let rc = std::rc::Rc::new(5);
    tokio::spawn(async move {
        println!("{rc}"); // compile error
    });
}
```
**Compile error**: `Rc<i32>` cannot be shared between threads safely.

`tokio::spawn` requires the future to be `Send` (may run on any thread in the pool). `Rc` is not `Send`.

**Fix**: use `Arc` instead of `Rc`.

---

## `spawn_blocking` for CPU-Heavy Work

```rust
#[tokio::main]
async fn main() {
    let result = tokio::task::spawn_blocking(|| {
        compute_primes(1_000_000) // runs on a dedicated blocking thread pool
    }).await.unwrap();

    println!("{result}");
}
```

Never run CPU-heavy work directly in an async task — it starves the executor of threads for I/O work.

---

## Async vs Threads: When to Use What

| Use case | Recommendation |
|----------|----------------|
| Network I/O, database, file I/O | async |
| Heavy computation (image processing, crypto) | `spawn_blocking` or OS threads |
| Thousands of concurrent connections | async (tasks are much lighter than threads) |
| Simple parallel work on data | OS threads or rayon |
| Calling a blocking library | `spawn_blocking` |

---

## Demo: Building a Concurrent Counter

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let count = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&count);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *c.lock().unwrap() += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("final count: {}", count.lock().unwrap()); // 10000
}
```

Without `Mutex`, concurrent writes would be a data race — Rust won't let you compile that.
