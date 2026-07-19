// Q: Predict — this program's `for msg in rx` loop. With the `drop(tx)` line removed, does
//    it (a) finish, or (b) hang forever?

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx = tx.clone(); // one sender clone per producer
        thread::spawn(move || tx.send(i * 10).unwrap());
    }
    drop(tx); // ← drop the ORIGINAL sender — without this the loop below never ends

    for msg in rx {
        // yields messages until ALL senders are dropped
        println!("got {msg}");
    }
    println!("done");
}

// A: (b) Without `drop(tx)` it HANGS FOREVER. Iterating `rx` ends only when EVERY sender is
//    dropped. Each thread drops its clone when it finishes — but the ORIGINAL `tx` in main is
//    still alive, so the channel never "closes" and the loop waits for a message that never
//    comes. Dropping the original is the fix. (mpsc = multi-producer, single-consumer.)
//
// ── more Q&A ──
// Q: Channels vs Arc<Mutex<T>> — when do you pick which?
// A: Channels MOVE data between threads (message passing, no shared state to lock). Arc<Mutex>
//    SHARES one value in place. "Share memory by communicating" (channels) is often simpler
//    and avoids lock bugs.
// Q: Is std's channel bounded?
// A: `channel()` is unbounded (a fast producer can grow memory without limit). `sync_channel(n)`
//    is bounded — `send` blocks when full, giving you backpressure.
