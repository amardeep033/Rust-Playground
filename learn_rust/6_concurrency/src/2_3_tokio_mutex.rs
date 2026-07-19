// Q: Predict — is `tokio::sync::Mutex` always the right choice in async code, or only
//    sometimes? When would `std::sync::Mutex` still be better even inside async?

use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..3 {
        let d = data.clone();
        handles.push(tokio::spawn(async move {
            let mut g = d.lock().await; // async lock — yields the thread while waiting
            *g += 1;
        }));
    }
    for h in handles {
        h.await.unwrap();
    }
    println!("{}", *data.lock().await); // 3
}

// A: Only SOMETIMES. Use `tokio::sync::Mutex` when you must hold the lock ACROSS an `.await`
//    (e.g. an async DB call while holding it) — it suspends the task instead of blocking the
//    thread. But if you just lock, mutate, and unlock with NO await in between, `std::sync::
//    Mutex` is faster and perfectly fine even in async. Reaching for the async Mutex reflexively
//    is a common over-correction.
//
// ── more Q&A ──
// Q: Why is holding a std Mutex guard across `.await` dangerous?
// A: The guard blocks the worker thread; if your task parks there mid-await, other tasks on
//    that thread stall, and lock-ordering across suspended tasks can deadlock the runtime.
// Q: The async Mutex is "slower" — so why ever use it?
// A: Because blocking a worker thread is far worse than a slightly heavier lock. Correctness
//    (not starving the executor) beats the micro-cost when you genuinely await under the lock.
