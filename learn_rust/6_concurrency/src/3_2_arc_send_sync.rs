// Q: Why does Rc fail to cross a thread boundary but Arc works? What do Send and
//    Sync mean?

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());

    // let rc = std::rc::Rc::new(0);
    // thread::spawn(move || { let _ = rc; }); // Rc is not Send → won't compile
}

// A: Send = ownership of a value may be MOVED to another thread; Sync = a &T may be
//    SHARED across threads. Rc's refcount is non-atomic, so it is neither, and the
//    compiler rejects it in spawn. Arc uses an atomic refcount, so it is Send+Sync (when
//    T is). `move` transfers each captured clone into its thread.
//
// ── more Q&A ──
// Q: Demo a data race — will this compile, and how do you fix it?
//        let mut n = 0;
//        thread::spawn(|| n += 1);   // thread A
//        thread::spawn(|| n += 1);   // thread B
//    A: It does NOT compile — the closures borrow `n` mutably from two threads, which
//    safe Rust rejects (a data race). Fix by synchronizing the shared state: wrap it in
//    Arc<Mutex<i32>> (as above) so writes are serialized, or use an atomic (AtomicI32)
//    with fetch_add for a lock-free counter.
// Q: Data race vs race condition?
// A: A data race = unsynchronized concurrent memory access with ≥1 write — a MEMORY-
//    SAFETY bug that safe Rust prevents. A race condition = any timing/ordering-dependent
//    LOGIC bug — broader, and Rust does NOT prevent it. All data races are race
//    conditions, but not all race conditions are data races.
// Q: Does Rust prevent deadlocks?
// A: No — deadlock and lock contention are the programmer's responsibility (consistent
//    lock ordering, etc.). Rust only rules out data races.
// Q: When would you reach for an atomic instead of Arc<Mutex<T>>?
// A: For a single primitive counter/flag: Arc<AtomicI32/AtomicBool> with fetch_add/
//    load/store is lock-free and cheaper than locking a whole Mutex.
