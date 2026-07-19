// Q: Predict — with `#[derive(Debug)]` on Point, which line fails: `{p:?}` or `{p}`?

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Display must be written by hand — it is NOT derivable
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Meters(f64);
impl From<f64> for Meters {
    fn from(v: f64) -> Self {
        Meters(v)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{p:?}"); // ✅ Debug — derived
    println!("{p}"); // ✅ only because we hand-wrote Display; ❌ E0277 without that impl
    println!("{}", p.clone() == p); // ✅ Clone + PartialEq — derived

    let m: Meters = 5.0.into(); // Into came free from `impl From`
    println!("{}", m.0);
}

// A: `{p}` (Display) is the one that would fail — `#[derive(Debug)]` only gives you `{:?}`.
//    Display can NOT be derived; you must implement `fmt::Display` yourself (as above). The
//    split is deliberate: Debug is for developers (`{:?}`), Display is a human-facing choice
//    only you can make. Forgetting this and reaching for `{}` on a struct is a classic E0277.
//
// ── more Q&A ──
// Q: Which common traits ARE derivable?
// A: Debug, Clone, Copy, PartialEq/Eq, Hash, Default, PartialOrd/Ord. Display and most
//    behaviour traits are not — derive covers the "mechanical" ones.
// Q: Why implement `From` and never `Into`?
// A: A blanket impl gives you `Into` for free whenever you impl `From`. So write `From`,
//    get both. Bonus: `?` uses this `From` to convert an error into your function's error type.
