// Q: Why does Rust have both Eq/PartialEq and Ord/PartialOrd?
//    Isn't equality just equality, and ordering just sorting?

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug, PartialEq)]
struct Measurement {
    value: f64,
}

impl PartialOrd for Measurement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

fn main() {
    let a = Version {
        major: 1,
        minor: 2,
        patch: 0,
    };
    let b = Version {
        major: 1,
        minor: 3,
        patch: 0,
    };

    println!("version equality: {}", a == b);
    println!("version ordering: {:?}", a.cmp(&b));
    println!("version less-than: {}", a < b);

    let temp = Measurement { value: 37.0 };
    let weird = Measurement { value: f64::NAN };

    println!(
        "normal partial_cmp: {:?}",
        temp.partial_cmp(&Measurement { value: 40.0 })
    );
    println!("nan equals itself? {}", weird == weird);
    println!("nan partial_cmp itself: {:?}", weird.partial_cmp(&weird));

    let mut versions = vec![
        Version {
            major: 2,
            minor: 0,
            patch: 0,
        },
        Version {
            major: 1,
            minor: 9,
            patch: 9,
        },
        Version {
            major: 1,
            minor: 10,
            patch: 0,
        },
    ];

    versions.sort();
    println!("sorted versions: {versions:?}");
}

// A: `PartialEq` means `==` is available. `Eq` adds the promise that equality is
//    reflexive: every value equals itself. `PartialOrd` means values may be
//    ordered, but some pairs can be incomparable. `Ord` means every pair has one
//    total, stable ordering.
//
// -- more Q&A --
// Q: Why are floats not Eq or Ord?
// A: Because NaN breaks the normal rules: `NaN == NaN` is false, and comparing NaN
//    with another float returns None for `partial_cmp`.
// Q: Why does `sort()` need Ord?
// A: A sort algorithm needs to know how every pair should be ordered. If comparison
//    can return None, the algorithm has no universal answer.
// Q: When can I derive these?
// A: Derive them when all fields support the trait and the field order is the order
//    you want. Structs compare fields top to bottom. Tuples compare left to right.
