// Q: How do you gate a generic's methods by bounds, use a where clause, and
//    implement a trait for ALL types meeting a bound (blanket impl)?

use std::fmt::Display;

struct Pair<T> {
    a: T,
    b: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn larger(&self) {
        if self.a >= self.b {
            println!("{}", self.a);
        } else {
            println!("{}", self.b);
        }
    }
}

fn show<T>(x: T)
where
    T: Display,
{
    println!("{x}");
}

trait Loud {
    fn loud(&self) -> String;
}
impl<T: Display> Loud for T {
    fn loud(&self) -> String {
        format!("{self}!!!")
    }
}

trait Named: Display {
    fn label(&self) -> String {
        format!("name={self}")
    }
}
impl Named for i32 {}

fn main() {
    Pair { a: 3, b: 7 }.larger();
    show("hi");
    println!("{}", 42.loud()); // blanket impl → works on i32
    println!("{}", 42.label()); // supertrait method
}

// A: Put bounds on the impl block (`impl<T: Display + PartialOrd>`) to gate methods to
//    qualifying types; a `where` clause is the same thing, tidier for many bounds. A
//    blanket impl (`impl<T: Display> Loud for T`) implements a trait for every type
//    meeting the bound.
//
// ── more Q&A ──
// Q: What is a supertrait?
// A: `trait Named: Display` — implementors of Named must also implement Display, and
//    Named's methods may rely on Display.
// Q: Real-world blanket impl example?
// A: std's `impl<T: Display> ToString for T` — every Display type gets `.to_string()`
//    for free.
