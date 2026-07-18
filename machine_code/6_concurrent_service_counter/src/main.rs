use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let batches: Vec<Vec<String>> = vec![
        vec!["auth".into(), "auth".into(), "billing".into()],
        vec!["billing".into(), "search".into()],
        vec!["auth".into(), "search".into(), "search".into()],
    ];

    let counter = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = Vec::new();

    for batch in batches {
        let local_cntr_ref = counter.clone();

        let local_handler = tokio::spawn(async move {
            for job in batch {
                let mut lock = local_cntr_ref.lock().await;
                lock.entry(job).and_modify(|c| *c += 1).or_insert(1);
            }
        });
        handles.push(local_handler);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let counter = Arc::try_unwrap(counter).expect("Couldnt get the value").into_inner();

    for (job, count) in counter {
        println!("{job} : {count}");
    }
}
