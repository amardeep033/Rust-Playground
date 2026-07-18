// Q: How do you get {:?} vs {} printing, and how do From/Into conversions work?

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

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
    println!("{p:?}"); // Debug (derived)
    println!("{p}"); // Display (hand-written)
    println!("{}", p.clone() == p); // Clone + PartialEq (derived)

    let m: Meters = 5.0.into(); // Into, auto-derived from From
    let m2 = Meters::from(9.0);
    println!("{} {}", m.0, m2.0);
}

// A: `#[derive(Debug)]` gives `{:?}`; Display (`{}`) must be implemented by hand via
//    fmt::Display. Implement `From<T>` and you get `Into` for free.
//
// ── more Q&A ──
// Q: Which common traits are derivable, and which is NOT?
// A: Derivable: Debug, Clone, PartialEq/Eq, Hash, Default, PartialOrd/Ord. Display is
//    NOT derivable — you always write it.
// Q: From or Into — which do you implement?
// A: Implement From; Into comes free via a blanket impl. Prefer From bounds in APIs.
// Q: How does `?` convert one error type into another?
// A: It calls `.into()`, which uses your `From` impl to convert the source error.
