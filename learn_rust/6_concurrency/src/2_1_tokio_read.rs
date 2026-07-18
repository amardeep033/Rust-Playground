// Q: Do tokio::spawn tasks need `.await` to run? What happens if main returns first?

use std::sync::Arc;

#[derive(Debug)]
struct Msg {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let list = Arc::new(vec![
        Msg { id: 1, name: "Amar".into() },
        Msg { id: 2, name: "Deep".into() },
    ]);

    let a = list.clone();
    let h1 = tokio::spawn(async move { println!("{:?}", a[0]) });

    let b = list.clone();
    let h2 = tokio::spawn(async move { println!("{:?}", b[1]) });

    h1.await.unwrap();
    h2.await.unwrap();
}

// A: No — a spawned task starts running on the runtime immediately, in the background;
//    awaiting its JoinHandle only WAITS for the result, it doesn't start it. But if
//    main returns while tasks are unfinished, the runtime shuts down and drops them
//    mid-flight — so these awaits guarantee both actually print.
//
// ── more Q&A ──
// Q: What bound must a future satisfy for tokio::spawn?
// A: `Future + Send + 'static` — it may run on any worker thread and outlive the spawner,
//    so it can't borrow non-'static locals (hence Arc, not &).
// Q: Does awaiting the JoinHandle "start" the task?
// A: No. The task was already running; `.await` just waits for and unwraps its output.
