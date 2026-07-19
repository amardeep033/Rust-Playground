// Q: A read fails deep in your app. Predict — what extra does `.context("..")` give you in
//    the printed error, and what's the difference between printing it with `{}` vs `{:#}`?

use anyhow::{Context, Result};

fn read_port(raw: &str) -> Result<u16> {
    let port: u16 = raw
        .trim()
        .parse()
        .context("port must be a number")?; // attach a human message to whatever error `?` sees
    Ok(port)
}

fn main() -> Result<()> {
    for raw in ["8080", "notaport"] {
        match read_port(raw) {
            Ok(p) => println!("{raw:?}: ok {p}"),
            Err(e) => println!("{raw:?}: {e:#}"), // {:#} shows the FULL chain: "port must be a number: invalid digit..."
        }
    }
    let p = read_port("9000")?; // `?` in main works because main returns anyhow::Result<()>
    println!("final {p}");
    Ok(())
}

// A: `.context("port must be a number")` wraps the low-level error with a readable layer, so
//    you get "port must be a number: invalid digit found in string" instead of a bare parse
//    error. `{}` prints only the TOP message; `{:#}` prints the whole context CHAIN (each
//    `.context` layer joined by ": ") — that trail is anyhow's whole value for debugging apps.
//
// ── more Q&A ──
// Q: When should code `panic!` instead of returning a Result/anyhow error?
// A: Only for BUGS / broken invariants (unreachable states, failed assertions). Expected
//    failures — bad input, missing file, parse error — are `Result`. Library code should
//    almost never panic; it robs the caller of the choice.
// Q: Does using anyhow mean you can't have typed errors anywhere?
// A: No — they compose. Libraries define typed `thiserror` enums; the top-level app collects
//    them into `anyhow::Error` with `?` and adds context. Typed at the edges, dynamic at the top.
