use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <mb_per_step>", args[0]);
        std::process::exit(1);
    }

    // read MB from arg position 1
    let size_mb: usize = args[1]
        .parse()
        .expect("Arg1 must be integer MB value");

    let sleep_time: u64 = args[2]
        .parse()
        .expect("sleep time not provided");

    println!("PID: {}", std::process::id());
    println!("Allocating {} MB per step...", size_mb);

    let mut data: Vec<Vec<u8>> = Vec::new();

    // allocate 10 times
    for i in 1..=10 {
        let bytes = size_mb * 1024 * 1024;

        let mut block = Vec::with_capacity(bytes);
        block.resize(bytes, 1u8);

        data.push(block);

        println!("Allocated {} MB total", i * size_mb);
        thread::sleep(Duration::from_secs(2));
    }

    println!("Holding memory for observation...");
    thread::sleep(Duration::from_secs(sleep_time));

    println!("Final len: {}", data.len());
}
