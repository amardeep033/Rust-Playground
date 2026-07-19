// Q: Predict — two threads want to read the same Vec. Can you just capture it by reference
//    in both closures, or do you need Arc?

use std::sync::Arc;
use std::thread;

fn main() {
    let list = Arc::new(vec![10, 20, 30]);

    // let h1 = thread::spawn(|| println!("{:?}", list)); // ❌ closure may outlive main → can't borrow a local
    let a = Arc::clone(&list); // cheap: bumps refcount, does NOT copy the Vec
    let h1 = thread::spawn(move || println!("{:?}", a[0]));

    let b = Arc::clone(&list);
    let h2 = thread::spawn(move || println!("{:?}", b[2]));

    h1.join().unwrap();
    h2.join().unwrap();
}

// A: You need Arc. A thread may outlive the stack frame it was spawned from, so it can't
//    borrow a local — the borrow could dangle. `Arc` (Atomic Reference Counted) gives each
//    thread a shared OWNED handle; `Arc::clone` just bumps an atomic counter (no data copy),
//    and the Vec is freed when the last handle drops. No lock needed here — it's read-only.
//
// ── more Q&A ──
// Q: What does `join().unwrap()` do — why the unwrap?
// A: `join()` blocks until the thread finishes and returns a `Result`; `unwrap()` re-surfaces
//    a panic that happened INSIDE the thread (otherwise it's silently swallowed).
// Q: Real OS thread vs a tokio task — when each?
// A: OS thread for heavy/blocking CPU work (a real kernel thread, own stack). Tokio task for
//    many concurrent I/O jobs (lightweight, thousands multiplexed onto a few worker threads).
