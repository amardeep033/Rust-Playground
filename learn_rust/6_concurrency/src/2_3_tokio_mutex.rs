// Q: When do you actually need an async Mutex (tokio::sync::Mutex)?

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
struct Msg {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let list = Arc::new(Mutex::new(vec![
        Msg { id: 1, name: "Amar".into() },
        Msg { id: 2, name: "Deep".into() },
    ]));

    let a = list.clone();
    let h1 = tokio::spawn(async move {
        a.lock().await[0].id = 3;
    });

    let b = list.clone();
    let h2 = tokio::spawn(async move {
        b.lock().await[1].id = 4;
    });

    h1.await.unwrap();
    h2.await.unwrap();
    println!("{:?}", list.lock().await);
}

// A: Only when you must hold the lock ACROSS an `.await` point — the async Mutex yields
//    the thread while waiting instead of blocking it.
//
// ── more Q&A ──
// Q: If you don't await while holding the lock, which Mutex should you use?
// A: The plain std::sync::Mutex — it's faster, and holding it briefly (no await inside)
//    is fine even in async code.
// Q: Why is holding a std Mutex across `.await` dangerous?
// A: The guard blocks the OS thread; if the task is parked there, other tasks on that
//    thread stall, and you can deadlock the runtime.
