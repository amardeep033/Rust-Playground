// Q: The caller needs to react differently to "not found" vs "bad input". Predict — why
//    is `Box<dyn Error>` a bad fit here, and what does `#[from]` do that saves you a
//    `map_err`?

use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("user not found: id={0}")]
    NotFound(u32),
    #[error("permission denied")]
    Unauthorized,
    #[error("bad number: {0}")]
    Parse(#[from] std::num::ParseIntError), // #[from] auto-generates From<ParseIntError>
}

fn find_user(id: u32) -> Result<String, AppError> {
    match id {
        1 => Ok("Amar".into()),
        0 => Err(AppError::Unauthorized),
        _ => Err(AppError::NotFound(id)),
    }
}

fn parse_id(s: &str) -> Result<String, AppError> {
    let id: u32 = s.parse()?; // #[from] lets `?` convert ParseIntError → AppError::Parse automatically
    find_user(id)
}

fn main() {
    for input in ["1", "0", "99", "abc"] {
        match parse_id(input) {
            Ok(name) => println!("{input}: {name}"),
            Err(AppError::NotFound(id)) => println!("{input}: no user {id}"), // caller MATCHES the variant
            Err(e) => println!("{input}: {e}"),
        }
    }
}

// A: `Box<dyn Error>` erases the type, so the caller can't cleanly `match` on the cause.
//    A typed enum keeps each failure as a named variant the caller can pattern-match.
//    `#[from]` generates `From<ParseIntError> for AppError`, so `?` converts the source
//    error into `AppError::Parse` automatically — without it you'd write
//    `.map_err(AppError::Parse)?` by hand on every call. This is the LIBRARY style.
//
// ── more Q&A ──
// Q: What does `#[error("..")]` actually generate?
// A: The `Display` impl for that variant — the message printed by `{}`. `#[derive(Error)]`
//    also wires up the `Error` trait (and `source()` for `#[from]` fields).
// Q: thiserror or anyhow — how do you choose in one sentence?
// A: Writing a LIBRARY whose callers branch on errors → thiserror (typed). Writing an
//    APP that just logs/propagates → anyhow (dynamic). They compose: libs export thiserror
//    enums, the app collects them with anyhow.
