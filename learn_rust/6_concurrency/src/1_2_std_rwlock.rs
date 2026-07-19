// Q: Reads vastly outnumber writes on shared state. Predict — with a RwLock, can two
//    threads hold `.read()` at the same time? Can two hold `.write()`?

use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let a = Arc::clone(&data);
    let h1 = thread::spawn(move || {
        let r = a.read().unwrap(); // many readers allowed simultaneously
        println!("reader sees {:?}", *r);
    });

    let b = Arc::clone(&data);
    let h2 = thread::spawn(move || {
        let mut w = b.write().unwrap(); // writer needs EXCLUSIVE access — waits for all readers
        w.push(4);
    });

    h1.join().unwrap();
    h2.join().unwrap();
    println!("{:?}", data.read().unwrap());
}

// A: Multiple `.read()` holders — YES (that's the point). Multiple `.write()` — NO, a writer
//    is exclusive: it waits until every reader AND writer has released. So RwLock = "many
//    readers OR one writer". It wins when reads dominate; the cost is that it must track a
//    reader count, making it heavier than a plain Mutex.
//
// ── more Q&A ──
// Q: Reads and writes are roughly 50/50 — RwLock or Mutex?
// A: Mutex. RwLock's reader-count bookkeeping only pays off when reads massively outnumber
//    writes; otherwise the simpler, cheaper Mutex wins.
// Q: Can holding a read lock while trying to take a write lock deadlock?
// A: Yes — if the same thread (or a cycle of threads) holds a read and then requests a write
//    on the same lock, it can deadlock. Release before upgrading; don't nest lock acquisitions.
