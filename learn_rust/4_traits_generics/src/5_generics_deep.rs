// Q: Predict — `Pair<i32>` can call `.larger()`, but `Pair<SomeType>` can't. What decides
//    whether the method even exists?

use std::fmt::Display;

struct Pair<T> {
    a: T,
    b: T,
}

// method exists ONLY for T that are Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn larger(&self) {
        if self.a >= self.b {
            println!("{}", self.a);
        } else {
            println!("{}", self.b);
        }
    }
}

// blanket impl: give EVERY Display type a `.loud()`
trait Loud {
    fn loud(&self) -> String;
}
impl<T: Display> Loud for T {
    fn loud(&self) -> String {
        format!("{self}!!!")
    }
}

// supertrait: Named requires Display too
trait Named: Display {
    fn label(&self) -> String {
        format!("name={self}")
    }
}
impl Named for i32 {}

fn main() {
    Pair { a: 3, b: 7 }.larger(); // ✅ i32 is Display + PartialOrd
    // Pair { a: vec![1], b: vec![2] }.larger(); // ❌ Vec isn't Display → method doesn't exist
    println!("{}", 42.loud()); // ✅ blanket impl reached i32 via Display
    println!("{}", 42.label()); // ✅ supertrait method
}

// A: The bound on the `impl` block decides. `impl<T: Display + PartialOrd> Pair<T>` means
//    `.larger()` EXISTS only for T meeting those bounds — for `Pair<Vec<i32>>` the method
//    simply isn't there (Vec isn't Display), and you get "method not found", not a body
//    error. Bounds gate capability per concrete type; this is how one generic type can
//    expose different methods to different T.
//
// ── more Q&A ──
// Q: What's a real blanket impl you use every day?
// A: `impl<T: Display> ToString for T` in std — that's why every Display type has
//    `.to_string()`. Our `Loud` is the same pattern.
// Q: What does a supertrait (`trait Named: Display`) buy you?
// A: It lets `Named`'s methods rely on Display being present, and forces implementors to
//    provide Display too — a way to say "you can't be Named unless you're also Display".
