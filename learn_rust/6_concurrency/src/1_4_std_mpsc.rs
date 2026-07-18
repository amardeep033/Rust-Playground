// Q: Fan work out to N producer threads feeding one consumer. Why is `drop(tx)`
//    needed?

use std::sync::mpsc;
use std::thread;

fn main() {
    let items = vec![1, 2, 3, 4, 5];
    let (tx, rx) = mpsc::channel();

    let producers = 2;
    let chunk = items.len().div_ceil(producers);
    let mut iter = items.into_iter();
    let mut handles = vec![];

    for _ in 0..producers {
        let batch: Vec<_> = iter.by_ref().take(chunk).collect();
        if batch.is_empty() {
            break;
        }
        let tx = tx.clone();
        handles.push(thread::spawn(move || tx.send(batch).unwrap()));
    }
    drop(tx);

    for batch in rx {
        println!("{batch:?}");
    }
    for h in handles {
        h.join().unwrap();
    }
}

// A: mpsc = multi-producer, single-consumer: clone tx once per producer, keep the one
//    rx. Iterating `rx` blocks and yields messages until EVERY sender is dropped. Each
//    thread drops its clone when it finishes, but the ORIGINAL tx in main also counts —
//    without `drop(tx)` the loop waits forever for a sender that never closes.
//
// ── more Q&A ──
// Q: What does mpsc stand for?
// A: Multi-producer, single-consumer — many `tx` clones, exactly one `rx`.
// Q: How do you share state between threads instead of passing messages?
// A: Arc<Mutex<T>> / Arc<RwLock<T>>. Channels move data; locks share it in place.
// Q: Is std::mpsc bounded?
// A: The default channel() is unbounded; sync_channel(n) gives a bounded one that
//    blocks the sender when full (backpressure).
