// Q: Predict — `let fut = fetch(1);` (no `.await`). Has the sleep/work inside `fetch`
//    started running yet? And why is `std::thread::sleep` a bug inside an async fn?

use std::time::Duration;

async fn fetch(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(50)).await; // async sleep: suspends the TASK
    format!("data-{id}")
}

#[tokio::main]
async fn main() {
    let fut = fetch(1); // nothing has run yet — a Future is inert
    println!("built the future, still nothing happened");
    println!("{}", fut.await); // NOW it runs to completion

    // std::thread::sleep(Duration::from_secs(1)); // ❌ would freeze the whole executor thread
    let heavy = tokio::task::spawn_blocking(|| (1..=1_000_000u64).sum::<u64>()) // CPU work off the async pool
        .await
        .unwrap();
    println!("{heavy}");
}

// A: Nothing has run — an `async fn` returns a LAZY Future that does zero work until it's
//    polled by `.await` (or a spawn). It is NOT a thread. `std::thread::sleep` (or any blocking/
//    CPU-bound call) PARKS the executor's worker thread, freezing every other task scheduled on
//    it. Use `tokio::time::sleep` for waits and `spawn_blocking` for CPU-heavy work.
//
// ── more Q&A ──
// Q: If futures don't run until awaited, how do multiple things run "concurrently"?
// A: A future is a state machine the executor POLLS. `.await` yields at each suspension point,
//    so the executor interleaves many futures on one thread — concurrency without extra threads.
// Q: Where does genuinely CPU-heavy work belong?
// A: `spawn_blocking` (a dedicated blocking thread pool) or a real thread. Never inline in an
//    async task, or it starves the executor and stalls unrelated I/O tasks.
