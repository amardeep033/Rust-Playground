// Q: A fast producer feeds a slow consumer through `mpsc::channel(2)`. Predict — what does
//    the producer's `tx.send(...).await` do once 2 items are sitting unconsumed?

use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(2); // capacity 2 = bounded

    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap(); // BLOCKS (suspends) here when the buffer is full
            println!("sent {i}");
        }
    });

    while let Some(v) = rx.recv().await {
        println!("  got {v}");
        tokio::time::sleep(Duration::from_millis(20)).await; // slow consumer
    }
    producer.await.unwrap();
}

// A: `send().await` SUSPENDS the producer once the 2-slot buffer is full, resuming only after
//    the consumer drains a slot. That's backpressure — the channel automatically paces the
//    producer to the consumer's speed. (std's `channel()` is unbounded and can't do this;
//    that's the key difference from tokio's bounded, async channel.)
//
// ── more Q&A ──
// Q: What capacity should the channel be — how do you decide?
// A: No magic number; size it to tolerable in-flight work. Small (1–few) = tight pacing, low
//    memory. Larger = absorbs bursts but hides a slow consumer and uses more memory. Start
//    small; raise only if the producer stalls on natural bursts. Roughly: peak burst size.
// Q: When does the `rx.recv().await` loop end?
// A: When ALL senders (every `tx` clone) are dropped — then `recv()` returns `None`.
