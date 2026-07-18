use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let vec_list = vec![1, 2, 3, 4, 5];

    let final_list = run_pipeline(vec_list).await;

    for num in final_list {
        println!("Final {num}");
    }
}

async fn run_pipeline(input: Vec<i32>) -> Vec<i32> {
    // Bounded channel (capacity 1): a full queue makes `send` block, so a slow
    // consumer applies backpressure to the producer instead of memory growing unbounded.
    let (tx1, mut rx1) = mpsc::channel(2);
    let (tx2, mut rx2) = mpsc::channel(2);

    // Task starts running as soon as it's spawned, not when `.await`ed later.
    // `h1` is just a handle for joining/checking status, not a trigger to start.
    let h1 = tokio::spawn(async move {
        //sends in order rcvd in queue:FIFO
        for num in input {
            tx1.send(num).await.unwrap();
        }
    });

    let h2 = tokio::spawn(async move {
        while let Some(message) = rx1.recv().await {
            println!("GOT = {}", message);
            //dont unwrapn- break - if err other will also stop since no cons tx1 stop -- since no prod tx2 stop(chained)
            tx2.send(message * 2).await.unwrap();
            //tx drop -- gracefully stops
        }
    });

    let h3 = tokio::spawn(async move {
        let mut final_rcvd = Vec::new();
        while let Some(message) = rx2.recv().await {
            println!("GOT2 = {}", message);
            final_rcvd.push(message);
        }
        final_rcvd
    });

    // Polls all three handles concurrently and waits for all to finish; since the
    // tasks are already running, this differs from sequential `.await`s only in
    // how the results are collected, not in when the work actually happens.
    let (producer_res, transformer_res, consumer_res) = tokio::join!(h1, h2, h3);

    // Each `Result` here is from `JoinHandle::await` — Err means the task panicked.
    producer_res.unwrap();
    transformer_res.unwrap();
    consumer_res.unwrap()
}
