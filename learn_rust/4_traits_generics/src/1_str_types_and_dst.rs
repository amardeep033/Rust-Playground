// Q: Predict — `greet` takes `&String`. Which of these three calls fails to compile, and
//    would switching the param to `&str` fix it?

fn greet(s: &str) {
    // vs `s: &String`
    println!("hi {s}");
}

fn main() {
    let owned = String::from("Amar");
    greet(&owned); // works either way
    greet("literal"); // ❌ if param is &String (a literal is &str, not &String) — ✅ with &str
    greet(&owned[0..2]); // ❌ if param is &String (a slice is &str) — ✅ with &str

    // the three string forms:
    let s: String = String::from("hello"); // (ptr,len,cap) owns heap
    let borrowed: &str = &s; // (ptr,len) borrows s
    let slice: &str = &s[1..3]; // "el" — a view, no copy
    println!("{s} {borrowed} {slice}");
}

// A: With `&String`, BOTH the literal and the slice calls fail — a literal and a slice are
//    `&str`, not `&String`. Switching the param to `&str` fixes all of them, because
//    `&String` auto-coerces to `&str` (Deref) but not vice-versa. So `&str` params accept
//    strictly more callers. Taking `&String` is a common beginner over-restriction.
//
// ── more Q&A ──
// Q: Why can't `str` (bare, no `&`) be a local variable?
// A: `str` is a DST — its size isn't known at compile time (text can be any length), so it
//    can't sit on the stack. It only exists behind a pointer: `&str`, `Box<str>`.
// Q: What are the three DSTs, and what do they have in common?
// A: `str`, `[T]` (slice), `dyn Trait`. All unsized → all used behind a pointer (`&`/`Box`/
//    `Rc`), and that pointer is "fat" (carries a length or a vtable alongside the address).
