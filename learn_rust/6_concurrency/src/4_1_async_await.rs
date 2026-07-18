// Q: Is a future doing work before you `.await` it? And why is std::thread::sleep bad
//    inside async?

use std::time::Duration;

async fn fetch(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    format!("data-{id}")
}

#[tokio::main]
async fn main() {
    println!("{}", fetch(1).await);

    let heavy = tokio::task::spawn_blocking(|| (1..=1_000_000u64).sum::<u64>())
        .await
        .unwrap();
    println!("{heavy}");
}

// A: No — an `async fn` returns a LAZY future that does nothing until polled by `.await`
//    (or tokio::spawn). `.await` suspends THIS task and lets the executor run others; it
//    is not a new thread. std::thread::sleep (or any blocking/CPU call) parks the whole
//    executor thread, freezing every task on it.
//
// ── more Q&A ──
// Q: Does async create threads?
// A: No. A future is a state machine; the runtime polls many futures on a small pool of
//    worker threads. New threads only appear via thread::spawn or spawn_blocking.
// Q: What does `.await` actually do?
// A: Polls the future; if it's not ready it SUSPENDS the task and yields the thread to
//    the executor, resuming when the future can make progress.
// Q: Where does CPU-heavy work belong?
// A: In spawn_blocking (a dedicated blocking pool) or a real thread — never inline in an
//    async task, or it starves the executor.
