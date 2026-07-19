// Q: Five threads each increment a shared counter. Predict — what makes this correct, and
//    when exactly is the lock released?

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = c.lock().unwrap(); // wait for the lock, get exclusive access
            *guard += 1;
        })); // ← guard DROPS here (end of scope) → lock released automatically
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap()); // always 5
}

// A: The Mutex serializes the increments — only one thread holds it at a time, so no two
//    `+= 1`s interleave. The lock is released when the `MutexGuard` DROPS, i.e. at the end of
//    the closure's scope (RAII) — you never manually unlock. Arc provides shared ownership
//    across threads; Mutex provides the safe mutation. You need both.
//
// ── more Q&A ──
// Q: What happens if one thread calls `c.lock()` twice without dropping the first guard?
// A: Deadlock — the second `lock()` waits forever for a guard this same thread still holds.
//    std::sync::Mutex is not reentrant.
// Q: For a plain counter, is a Mutex even the best tool?
// A: No — an atomic (`Arc<AtomicUsize>` with `fetch_add`) is lock-free and cheaper for a
//    single primitive. Reach for Mutex when you're guarding a compound value (Vec, HashMap, struct).
