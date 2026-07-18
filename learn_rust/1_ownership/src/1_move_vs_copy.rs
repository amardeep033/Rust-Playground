// Q: After `let b = a;`, is `a` still usable? Does the answer depend on the type?

fn takes(s: String) {
    println!("{s}");
}

fn main() {
    let x = 5;
    let y = x;
    println!("copy: {x} {y}");

    let a = String::from("hello");
    let b = a;
    // println!("{a}"); // E0382: borrow of moved value `a`
    println!("move: {b}");

    let owned = String::from("world");
    takes(owned);
    // println!("{owned}"); // E0382: value used after move
}

// A: Depends on the type. Copy types (i32, bool, char, f64, &T, tuples of Copy)
//    duplicate their bits on assignment, so `a` stays valid. Non-Copy types
//    (String, Vec, Box — anything owning heap memory) MOVE ownership to `b`,
//    leaving `a` invalid. Passing to a function moves the value the same way.
//
// ── more Q&A ──
// Q: Is `&T` Copy or Move?
// A: The reference itself is Copy (the pointer is copied); the data stays borrowed,
//    not duplicated.
// Q: Why isn't String Copy?
// A: It owns a heap buffer; an implicit deep copy would be expensive, so Rust makes
//    you write `.clone()` explicitly and moves by default.
// Q: When does a borrow end — at the closing `}`?
// A: No. With non-lexical lifetimes (NLL) a borrow ends at its LAST USE, which can
//    be well before the end of scope.
// Q: What four bugs does ownership prevent?
// A: use-after-free, double-free, dangling pointers, and data races.
