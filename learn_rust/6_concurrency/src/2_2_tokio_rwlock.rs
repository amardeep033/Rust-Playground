// Q: Predict — inside async code, what goes wrong if you use `std::sync::RwLock` and hold it
//    across an `.await`? Why does `tokio::sync::RwLock` exist?

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let a = data.clone();
    let h = tokio::spawn(async move {
        {
            let r = a.read().await; // .await the lock — suspends the TASK, not the thread
            println!("read {:?}", *r);
        }
        a.write().await.push(4);
    });

    h.await.unwrap();
    println!("{:?}", *data.read().await);
}

// A: A std lock BLOCKS the OS thread while it waits. In async, many tasks share a few worker
//    threads — so a blocked thread freezes every other task on it, and if you hold a std guard
//    across `.await` you can deadlock the runtime. `tokio::sync::RwLock`'s `.read()/.write()`
//    are async: they SUSPEND the task and free the thread for others. Same many-readers-or-one-
//    writer semantics, but await-friendly.
//
// ── more Q&A ──
// Q: So is std::sync::Mutex/RwLock ever OK in async code?
// A: Yes — if you lock, touch the data, and unlock WITHOUT awaiting in between, a std lock is
//    faster and fine. The rule is only: never hold a std guard across an `.await`.
// Q: What does `.await` on the lock actually do while waiting?
// A: Returns control to the executor so it can poll other tasks; your task resumes once the
//    lock is free. The thread is never parked.
