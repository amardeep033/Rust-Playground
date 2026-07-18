// Q: Inside async code, why use tokio::sync::RwLock instead of std's RwLock?

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
struct Msg {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let list = Arc::new(RwLock::new(vec![
        Msg { id: 1, name: "Amar".into() },
        Msg { id: 2, name: "Deep".into() },
    ]));

    let a = list.clone();
    let h1 = tokio::spawn(async move {
        println!("{:?}", a.read().await[0]);
        a.write().await[0].id = 3;
    });

    let b = list.clone();
    let h2 = tokio::spawn(async move {
        println!("{:?}", b.read().await[1]);
        b.write().await[1].id = 4;
    });

    h1.await.unwrap();
    h2.await.unwrap();
}

// A: Its read()/write() are async and `.await` the lock instead of blocking the OS
//    thread. A std lock parks the whole executor thread while it waits, which can stall
//    every other task sharing that thread (and risks deadlock if held across an `.await`).
//
// ── more Q&A ──
// Q: What does `.await` on the lock actually do?
// A: Suspends THIS task (frees the thread for other tasks) until the lock is available,
//    rather than blocking the thread.
// Q: Is the reader/writer semantics different from std RwLock?
// A: No — many readers OR one writer, same as std; only the waiting mechanism is async.
