use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

extern crate tokio;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx.send("hello").await.unwrap();
    });

    tokio::select! {
        msg = rx.recv() => {
            println!("Got message: {:?}", msg);
        }
        _ = sleep(Duration::from_secs(1)) => {
            println!("Timeout happened");
        }
    }
}
