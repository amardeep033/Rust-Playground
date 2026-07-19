// Q: One function reads a file (io::Error) AND parses it (ParseIntError) — two different
//    error types. Predict — can a single `?` on each line work without a custom error enum?

use std::error::Error;

fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?; // std::io::Error
    let n: i32 = text.trim().parse()?; // std::num::ParseIntError
    Ok(n * 2)
}

fn main() {
    std::fs::write("/tmp/rust_num.txt", "21").unwrap();
    std::fs::write("/tmp/rust_bad.txt", "abc").unwrap();
    println!("{:?}", read_number("/tmp/rust_num.txt")); // Ok(42)
    println!("{:?}", read_number("/tmp/rust_bad.txt")); // Err(ParseIntError)
    println!("{:?}", read_number("/tmp/does_not_exist")); // Err(io ..)
}

// A: Yes — return `Result<T, Box<dyn Error>>` and BOTH errors flow through `?`. There's a
//    blanket `From<E: Error> for Box<dyn Error>`, so `?` boxes any error that implements
//    the `Error` trait. Zero boilerplate, no custom enum — the go-to for app code and quick
//    machine-coding. The trade-off is in the next question.
//
// ── more Q&A ──
// Q: What did you GIVE UP by boxing? Can the caller tell io-error from parse-error apart?
// A: Not easily — the concrete type is erased. To branch on the cause you must
//    `e.downcast_ref::<std::io::Error>()`, which is clumsy. If callers need to match the
//    cause, that's the signal to use a typed enum (thiserror) instead.
// Q: Box<dyn Error> vs anyhow?
// A: `anyhow::Error` is basically a nicer `Box<dyn Error>` — adds `.context("..")`, backtraces,
//    and better ergonomics. Reach for anyhow in real apps; Box<dyn Error> when you want std-only.
