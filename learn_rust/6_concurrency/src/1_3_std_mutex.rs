// Q: What does a Mutex guarantee, and when is it the better choice than RwLock?

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = c.lock().unwrap();
            *guard += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());
}

// A: A Mutex gives exactly ONE holder at a time exclusive access (reader or writer
//    alike). lock() returns a guard that releases automatically when it drops at end of
//    scope. It tracks no reader count, so it's cheaper than RwLock — prefer it when
//    writes are frequent or critical sections are short.
//
// ── more Q&A ──
// Q: When exactly is the lock released?
// A: When the MutexGuard is dropped — at end of scope, or explicitly via drop(guard).
// Q: What roles do Arc and Mutex each play in Arc<Mutex<T>>?
// A: Arc = shared OWNERSHIP across threads; Mutex = synchronized MUTATION. You need both.
// Q: What happens if one thread locks the same Mutex twice?
// A: Deadlock — the second lock() waits forever for a guard that thread still holds.
