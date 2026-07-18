// Q: How does tokio's mpsc differ from std's channel?

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Vec<i32>>(2);

    let items = vec![1, 2, 3, 4, 5];
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
        handles.push(tokio::spawn(async move { tx.send(batch).await.unwrap() }));
    }
    drop(tx);

    while let Some(batch) = rx.recv().await {
        println!("{batch:?}");
    }
    for h in handles {
        h.await.unwrap();
    }
}

// A: It's async and BOUNDED: the capacity is fixed at creation, and send().await
//    suspends the producer when the buffer is full (backpressure) rather than blocking a
//    thread. recv().await yields until a message arrives, ending once all senders drop —
//    so `drop(tx)` is still required, same as std.
//
// ── more Q&A ──
// Q: What is backpressure, and how does this channel provide it?
// A: Slowing producers to match a slow consumer. A bounded channel does it automatically:
//    once the buffer is full, send().await blocks until the consumer frees a slot.
// Q: When does the `while let Some(..) = rx.recv().await` loop end?
// A: When every sender (all tx clones AND the original) has been dropped — recv() then
//    returns None.
