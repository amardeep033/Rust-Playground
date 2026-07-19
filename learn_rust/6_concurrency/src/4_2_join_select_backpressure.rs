// Q: Two tasks take 80ms and 50ms. Predict — `join!(a, b)` total time: ~130ms (sum) or
//    ~80ms (max)? And what happens to the SLOWER future in a `select!`?

use std::time::Duration;

async fn work(name: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    format!("{name} done")
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(work("a", 80), work("b", 50)); // both run concurrently
    println!("{a}, {b}"); // ~80ms total, not 130ms

    tokio::select! {
        r = work("fast", 30) => println!("winner: {r}"), // first to finish wins...
        r = work("slow", 90) => println!("winner: {r}"), // ...the slow one is CANCELLED
    }
}

// A: `join!` = ~80ms (the MAX), because the two futures run concurrently and overlap their
//    waits — not 130ms (that would be awaiting them one after another). `select!` returns as
//    soon as the FIRST future completes and CANCELS the rest (drops them at their suspension
//    point) — here "fast" wins at 30ms and "slow" never finishes. Great for timeouts / "first
//    response wins".
//
// ── more Q&A ──
// Q: `join!` vs `tokio::spawn` — both run things concurrently, so what's the difference?
// A: `join!` drives futures concurrently ON THE CURRENT task (same thread, no Send needed).
//    `spawn` hands a task to the runtime to run INDEPENDENTLY (any worker thread, must be
//    Send + 'static). Use join! for "wait for these N together"; spawn for fire-and-track.
// Q: Is cancelling the slow future in select! safe — could it leave things half-done?
// A: The future is dropped at its last `.await`; its Drop impls run, but any work past that
//    await never happens. Don't rely on a select!-losable future to finish side effects.
