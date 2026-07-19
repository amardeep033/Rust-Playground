// Q: You must process a 50 GB log file on a machine with 8 GB RAM. Predict — does
//    `read_to_string(path)` work? What's the difference vs `BufReader::lines()`?

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let path = "/tmp/rust_demo_data.txt";

    // write with BufWriter (batches many small writes into few syscalls)
    let mut w = BufWriter::new(File::create(path)?);
    for i in 1..=10 {
        writeln!(w, "line {i}")?;
    }
    w.flush()?; // ← without this, buffered bytes may not reach disk before we read back

    // read_to_string(path)  → would load ALL 50 GB into memory → OOM ❌
    // BufReader::lines()    → streams ONE line at a time → constant memory ✅
    let reader = BufReader::new(File::open(path)?);
    let mut batch: Vec<String> = Vec::with_capacity(3);
    for line in reader.lines() {
        batch.push(line?);
        if batch.len() == 3 {
            println!("{batch:?}");
            batch.clear(); // process + release each batch → memory stays flat
        }
    }
    if !batch.is_empty() {
        println!("last {batch:?}");
    }
    Ok(())
}

// A: `read_to_string` loads the ENTIRE file into a String → 50 GB into 8 GB RAM = OOM.
//    `BufReader::lines()` yields one line at a time, so memory stays constant regardless of
//    file size. Batching (accumulate N, process, clear) lets you amortise work without ever
//    holding more than N lines. This "stream, don't slurp" instinct is what interviewers probe.
//
// ── more Q&A ──
// Q: Why wrap the File in BufReader/BufWriter at all?
// A: A raw File does a syscall per read/write. Buf* keeps an in-memory buffer and batches
//    them — turning thousands of syscalls into a handful. Huge for line-by-line work.
// Q: I wrote to a BufWriter but the file looks empty — why?
// A: The bytes are still in the buffer. `flush()` (or dropping the writer) forces them to
//    disk. Forgetting to flush before reading the same file back is a common bug.
