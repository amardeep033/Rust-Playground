// Q: When should you pick RwLock over Mutex?

use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Debug)]
struct Msg {
    id: i32,
    name: String,
}

fn main() {
    let list = Arc::new(RwLock::new(vec![
        Msg { id: 1, name: "Amar".into() },
        Msg { id: 2, name: "Deep".into() },
    ]));

    let a = Arc::clone(&list);
    let h1 = thread::spawn(move || {
        println!("{:?}", a.read().unwrap()[0]);
        a.write().unwrap()[0].id = 3;
    });

    let b = Arc::clone(&list);
    let h2 = thread::spawn(move || {
        println!("{:?}", b.read().unwrap()[1]);
        b.write().unwrap()[1].id = 4;
    });

    h1.join().unwrap();
    h2.join().unwrap();
    println!("{:?}", list.read().unwrap());
}

// A: RwLock allows MANY concurrent readers OR one exclusive writer, so it wins when
//    reads greatly outnumber writes.
//
// ── more Q&A ──
// Q: What's the cost of that flexibility vs a Mutex?
// A: RwLock tracks a reader count, so it carries more bookkeeping overhead than a plain
//    Mutex — not worth it if writes are frequent.
// Q: Many readers and a rare writer — which lock?
// A: RwLock. Frequent writes or tiny critical sections → Mutex.
// Q: Do read() / write() block?
// A: Yes — read() waits for any active writer to finish; write() waits for all readers
//    and writers to release.
