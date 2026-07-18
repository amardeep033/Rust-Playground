// Q: How do you define typed errors a caller can match on, and what does `#[from]`
//    buy you?

use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("user not found: id={0}")]
    NotFound(u32),
    #[error("permission denied")]
    Unauthorized,
    #[error("bad number: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn find_user(id: u32) -> Result<String, AppError> {
    match id {
        1 => Ok("Amar".into()),
        0 => Err(AppError::Unauthorized),
        _ => Err(AppError::NotFound(id)),
    }
}

fn parse_id(s: &str) -> Result<String, AppError> {
    let id: u32 = s.parse()?;
    find_user(id)
}

fn main() {
    for input in ["1", "0", "99", "abc"] {
        match parse_id(input) {
            Ok(name) => println!("{input}: {name}"),
            Err(AppError::NotFound(id)) => println!("{input}: no user {id}"),
            Err(e) => println!("{input}: {e}"),
        }
    }
}

// A: `#[derive(Error)]` builds a typed enum whose `#[error("..")]` strings become the
//    Display impl, and callers can `match` on individual variants. `#[from]` generates
//    `From<ParseIntError>`, so `?` auto-converts the source error into AppError::Parse.
//
// ── more Q&A ──
// Q: thiserror vs anyhow — when each?
// A: thiserror for LIBRARIES (typed variants callers match on); anyhow for
//    APPLICATIONS (one dynamic error you just surface).
// Q: What does the `#[error("..")]` attribute generate?
// A: The `Display` implementation for that variant — the message shown by `{}`.
