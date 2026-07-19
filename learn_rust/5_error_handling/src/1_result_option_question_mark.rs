// Q: Predict — does `?` work only on `Result`? And what must the enclosing function
//    return for `?` to be legal?

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?; // ? on Result: Err → early return
    Ok(n * 2)
}

fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?; // ? on OPTION: None → early return (surprises people)
    Some(c.to_ascii_uppercase())
}

fn main() {
    // fn main() { std::fs::read_to_string("x")?; } // ❌ E0277: main returns (), not Result/Option
    println!("{:?}", parse_and_double("21")); // Ok(42)
    println!("{:?}", parse_and_double("oops")); // Err(..)
    println!("{:?}", first_char_upper("hi")); // Some('H')
    println!("{:?}", first_char_upper("")); // None

    let name: Option<&str> = Some("amar");
    println!("{:?}", name.and_then(|n| (n.len() > 3).then(|| n.to_uppercase()))); // Some("AMAR")
    println!("{}", "notnum".parse::<i32>().unwrap_or(-1)); // -1
}

// A: `?` works on `Option` too — that's the common blind spot. What it needs is that the
//    ENCLOSING function returns the matching type (Result for a Result `?`, Option for an
//    Option `?`). On Result it desugars to `match { Ok(v)=>v, Err(e)=>return Err(e.into()) }`
//    — note `.into()`, which converts the error via `From` (how libraries unify error types).
//
// ── more Q&A ──
// Q: Why does `?` in `main` often fail to compile?
// A: Default `main` returns `()`, and `?` needs a Result/Option return. Change it to
//    `fn main() -> Result<(), Box<dyn Error>>` and `?` becomes legal in main.
// Q: match vs combinators — when each?
// A: `match`/`if let` when branches differ a lot or you handle both. Combinators
//    (`map`/`and_then`/`unwrap_or`/`ok_or`) for short linear transforms — less noise, same result.
