// Q: A function does file I/O AND parsing — two different error types. What's the
//    quickest way to return both from one function?

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
    println!("{:?}", read_number("/tmp/rust_bad.txt")); // Err(parse)
    println!("{:?}", read_number("/tmp/does_not_exist")); // Err(io)
}

// A: Return `Result<T, Box<dyn Error>>`. `?` auto-boxes ANY error that implements
//    std::error::Error, so io::Error and ParseIntError flow through the same return
//    type with zero boilerplate — the go-to for app code and quick machine-coding.
//
// ── more Q&A ──
// Q: Box<dyn Error> vs thiserror vs anyhow?
// A: Box<dyn Error> = std-only, quick, but the caller can't easily tell the cause.
//    thiserror = typed enum for LIBRARIES (callers match variants). anyhow ≈ a nicer
//    Box<dyn Error> for APPS (adds .context() and backtraces).
// Q: Can the caller tell WHICH error happened?
// A: Only by downcasting (`e.downcast_ref::<std::io::Error>()`), which is clumsy — if
//    callers must branch on the cause, use a typed enum (thiserror) instead.
// Q: Why does `?` accept two different error types here?
// A: Both implement std::error::Error, and there's a blanket `From<E: Error>` for
//    Box<dyn Error>, so `?` converts each into the boxed type.
// Q: Will `fn f() -> Result<i32, Box<dyn Error>> { Ok("x".parse()?) }` compile?
// A: Yes — ParseIntError: Error, so `?` boxes it. But `-> i32` (no Result) would NOT
//    compile: `?` needs the function to return Result/Option.
