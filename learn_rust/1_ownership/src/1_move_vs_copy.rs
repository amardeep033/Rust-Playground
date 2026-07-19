// Q: Predict — which of these two `println!`s compiles, and which doesn't? Why?

fn main() {
    let a = String::from("hello");
    let b = a;
    // println!("{a}"); // ❌ E0382: `a` was MOVED into `b` — String owns heap, so it moves
    println!("{b}"); // ✅

    let x = 5;
    let y = x;
    println!("{x} {y}"); // ✅ both valid — i32 is Copy, so `x` was duplicated, not moved
}

// A: The String line fails, the i32 line works — and the ONLY difference is the type.
//    Non-Copy types (own heap: String, Vec, Box) move on assignment, invalidating the
//    original. Copy types (i32, bool, char, f64, &T) duplicate their bits, so both stay
//    valid. The bug hides because both look like the same "let b = a" — the type decides.
//
// ── more Q&A ──
// Q: Would `takes(a)` (passing to a function) also move `a`, or only `let b = a`?
// A: Both. Passing a non-Copy value to a function moves it just like assignment — after
//    `takes(a)` you can't use `a`. Pass `&a` to lend it instead.
// Q: You keep hitting a move error — clone, borrow, or restructure?
// A: Borrow (`&a`) if the callee only reads → cheapest. Restructure to use the value
//    once if you can. `.clone()` last — it's a real heap copy, not free; reaching for it
//    reflexively is the common beginner smell.
