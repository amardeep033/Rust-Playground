// Q: Does `?` only work on Result? What does `?` desugar to?

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{:?}", parse_and_double("21"));
    println!("{:?}", parse_and_double("oops"));
    println!("{:?}", first_char_upper("hello"));
    println!("{:?}", first_char_upper(""));

    let name: Option<&str> = Some("amar");
    if let Some(n) = name {
        println!("hi {n}");
    }
    let long = name.and_then(|n| (n.len() > 3).then(|| n.to_uppercase()));
    println!("and_then: {long:?}");
}

// A: `?` works on Option too, not just Result — the enclosing fn must return the
//    matching type. On Result it desugars to
//    `match expr { Ok(v) => v, Err(e) => return Err(e.into()) }` (note `.into()`, so
//    the error is converted via From). On Option, None short-circuits to `return None`.
//
// ── more Q&A ──
// Q: What must a function return to use `?`?
// A: A Result (for `?` on Result) or an Option (for `?` on Option) — including
//    `fn main() -> Result<(), E>`.
// Q: Common Option combinators?
// A: map (transform Some), and_then (chain another Option), unwrap_or / _else / _default
//    (fallback), ok_or (→ Result), plus `if let Some(x)`.
// Q: How do you turn a Result into an Option?
// A: `res.ok()` — keeps Ok(v) as Some(v) and DROPS the error as None.
