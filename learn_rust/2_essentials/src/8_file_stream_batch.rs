// Q: Read a large file without loading it all into memory, processing lines in
//    batches of 3.

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let path = "/tmp/rust_demo_data.txt";

    let mut w = BufWriter::new(File::create(path)?);
    for i in 1..=10 {
        writeln!(w, "line {i}")?;
    }
    w.flush()?;

    let reader = BufReader::new(File::open(path)?);
    let mut batch: Vec<String> = Vec::with_capacity(3);
    for line in reader.lines() {
        batch.push(line?);
        if batch.len() == 3 {
            println!("{batch:?}");
            batch.clear();
        }
    }
    if !batch.is_empty() {
        println!("last {batch:?}");
    }
    Ok(())
}

// A: BufReader::lines() streams one line at a time, so memory stays constant no
//    matter the file size. Accumulate into a Vec and flush every N for batching (plus
//    the leftover < N at the end).
//
// ── more Q&A ──
// Q: Why wrap File in BufReader/BufWriter?
// A: Raw File does a syscall per read/write; the buffer batches them into far fewer
//    syscalls.
// Q: read_to_string vs lines()?
// A: read_to_string loads the WHOLE file into memory; lines() streams it, keeping
//    memory constant — required for huge files.
// Q: Why call flush() on the BufWriter?
// A: Buffered writes may still be sitting in memory; flush (or drop) forces them to
//    disk before we read the file back.
