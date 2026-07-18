// Q: When would you reach for anyhow instead of thiserror, and how do you attach
//    context?

use anyhow::{Context, Result};

fn read_port(raw: &str) -> Result<u16> {
    let port: u16 = raw.trim().parse().context("port must be a number")?;
    Ok(port)
}

fn main() -> Result<()> {
    for raw in ["8080", "  70  ", "notaport"] {
        match read_port(raw) {
            Ok(p) => println!("{raw:?}: ok {p}"),
            Err(e) => println!("{raw:?}: {e:#}"),
        }
    }

    let p = read_port("9000")?;
    println!("final: {p}");
    Ok(())
}

// A: anyhow is for APPLICATION/binary code where you just propagate and display errors
//    rather than match on them — one dynamic error type accepts any error through `?`.
//    `.context("..")` adds a human-readable layer (printed with `{:#}`).
//
// ── more Q&A ──
// Q: What must main return to use `?` inside it?
// A: `Result<(), E>` — e.g. `anyhow::Result<()>` or `Result<(), Box<dyn Error>>`.
// Q: `{:#}` vs `{}` on an anyhow error?
// A: `{:#}` prints the full context chain (each `.context` layer); `{}` prints just the top.
// Q: When should library code panic?
// A: Almost never — return a Result and let the caller decide. Panics are for bugs /
//    broken invariants, not expected failures.
