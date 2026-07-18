// Q: join! vs select!? And how does a bounded channel create backpressure?

use std::time::Duration;
use tokio::sync::mpsc;

async fn work(name: &str, ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(ms)).await;
    format!("{name} done")
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(work("a", 80), work("b", 50));
    println!("{a}, {b}");

    tokio::select! {
        r = work("fast", 30) => println!("{r}"),
        r = work("slow", 90) => println!("{r}"),
    }

    let (tx, mut rx) = mpsc::channel::<i32>(2);
    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            println!("sent {i}");
        }
    });
    while let Some(v) = rx.recv().await {
        println!("  got {v}");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    producer.await.unwrap();
}

// A: join! runs all its futures concurrently and waits for ALL (total time ≈ the
//    slowest). select! waits for the FIRST to finish and cancels the rest. A bounded
//    channel (capacity 2) applies backpressure: once the buffer is full, send().await
//    SUSPENDS the producer until the slow consumer drains an item.
//
// ── more Q&A ──
// Q: With join!, is total time the sum or the max of the futures?
// A: Roughly the MAX — they run concurrently, overlapping their waits.
// Q: What happens to the losing futures in select!?
// A: They're dropped/cancelled at the point they were suspended. Useful for timeouts and
//    "first response wins".
// Q: join! vs tokio::spawn — difference?
// A: join! runs futures on the CURRENT task concurrently; spawn hands a task to the
//    runtime to run independently (and it must be Send + 'static).
// Q: What capacity should the bounded channel have for backpressure — how do you decide?
// A: There's no magic number; size it to how much in-flight work you can tolerate.
//    Small (1–few) = tight backpressure, low memory, producer paced closely to the
//    consumer. Larger = absorbs bursts but uses more memory and hides a slow consumer.
//    Rule of thumb: start small, raise only if the producer stalls on natural bursts;
//    capacity ≈ (peak burst size) or (consumer throughput × acceptable latency).
