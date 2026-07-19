// Q: Predict — spawn a thread that captures an `Rc`. Does it compile? What single change
//    makes it work, and what trait is behind the error?

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // let rc = std::rc::Rc::new(0);
    // thread::spawn(move || { let _ = rc; }); // ❌ `Rc` cannot be sent between threads — not Send

    let counter = Arc::new(Mutex::new(0)); // swap Rc→Arc, add Mutex to mutate safely
    let mut handles = vec![];
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || *c.lock().unwrap() += 1));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap()); // 5

    // Demo a data race — will THIS compile?
    //     let mut n = 0;
    //     thread::spawn(|| n += 1);
    //     thread::spawn(|| n += 1);
    // ❌ No — sharing `&mut n` across threads is a data race; safe Rust rejects it at compile time.
}

// A: The Rc version does NOT compile — `Rc` is not `Send`, so it can't cross a thread boundary.
//    Swapping it for `Arc` fixes it (`Arc` is Send+Sync via an atomic refcount). The trait
//    behind the error is `Send`: it marks types whose OWNERSHIP can move to another thread.
//    (`Sync` marks types whose `&T` can be SHARED across threads.)
//
// ── more Q&A ──
// Q: Data race vs race condition?
// A: A data race = unsynchronized concurrent access, ≥1 write, same memory — a MEMORY-SAFETY
//    bug that safe Rust prevents (see the commented demo). A race condition = any timing/order-
//    dependent LOGIC bug — broader, and Rust does NOT prevent it. All data races are race
//    conditions; not all race conditions are data races.
// Q: Does Rust prevent deadlocks?
// A: No. Deadlock and lock contention are on you (consistent lock ordering, etc.). Rust's
//    guarantee is specifically "no data races", not "no concurrency bugs".
