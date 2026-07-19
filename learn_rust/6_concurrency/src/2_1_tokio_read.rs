// Q: Predict — you `tokio::spawn` two tasks but DON'T await their handles, and main returns
//    immediately after. Do the tasks' `println!`s reliably run?

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let list = Arc::new(vec![10, 20, 30]);

    let a = list.clone();
    let h1 = tokio::spawn(async move { println!("{}", a[0]) });
    let b = list.clone();
    let h2 = tokio::spawn(async move { println!("{}", b[2]) });

    // Remove these two awaits and main can return before the tasks print → output is lost.
    h1.await.unwrap();
    h2.await.unwrap();
}

// A: Not reliably. A spawned task starts running in the BACKGROUND immediately (you don't need
//    to await it for it to make progress) — but when `main` returns, the tokio runtime shuts
//    down and DROPS any unfinished tasks mid-flight. Awaiting the handles here keeps main alive
//    until both finish. So: spawn = "start now"; `.await` on the handle = "wait for the result".
//
// ── more Q&A ──
// Q: What bound must the spawned future satisfy, and why can't it borrow a local?
// A: `Future + Send + 'static`. It may run on any worker thread and outlive the spawner, so it
//    can't hold a non-'static borrow — hence Arc (owned), not `&`.
// Q: Does awaiting the JoinHandle "start" the task?
// A: No. The task was already running; `.await` only waits for it to finish and yields its output.
