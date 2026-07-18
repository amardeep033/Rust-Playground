// Q: How do two OS threads share the same read-only data? Why Arc and not just
//    clone the Vec?

use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct Msg {
    id: i32,
    name: String,
}

fn main() {
    let list = Arc::new(vec![
        Msg { id: 1, name: "Amar".into() },
        Msg { id: 2, name: "Deep".into() },
    ]);

    let a = Arc::clone(&list);
    let h1 = thread::spawn(move || println!("{:?}", a[0]));

    let b = Arc::clone(&list);
    let h2 = thread::spawn(move || println!("{:?}", b[1]));

    h1.join().unwrap();
    h2.join().unwrap();
}

// A: Wrap the data in Arc (atomic reference-counted pointer) and give each thread its
//    own clone. Arc::clone bumps a refcount, it does NOT copy the Vec, so all threads
//    read one shared allocation, freed only when the last handle drops. No lock is
//    needed because access is read-only.
//
// ── more Q&A ──
// Q: What does join() return, and what does unwrap() do here?
// A: A Result; unwrap() surfaces (as a panic) the case where the spawned thread itself
//    panicked, otherwise yields its return value.
// Q: Why is `move` required on the closure?
// A: The thread may outlive main's stack frame, so the closure must OWN its captured
//    Arc handle rather than borrow it.
// Q: std::thread vs a tokio task — when each?
// A: std::thread = a real OS thread, good for heavy/blocking CPU work; a tokio task is
//    lightweight, good for many concurrent I/O-bound jobs.
