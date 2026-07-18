// Q: Why does a struct that stores a `&str` need a `<'a>` annotation?

struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let novel = String::from("Call me Amar. Some years ago...");
    let first = novel.split('.').next().unwrap();
    let e = Excerpt { text: first };
    println!("{}", e.text);
}

// A: The struct borrows data it doesn't own, so it must never outlive that data.
//    `<'a>` ties the Excerpt's lifetime to the borrowed &str, letting the compiler
//    reject any Excerpt still used after `novel` is dropped.
//
// ── more Q&A ──
// Q: How do you avoid the lifetime annotation entirely?
// A: Own the data — make the field a `String` instead of `&str`. No borrow, no lifetime.
// Q: Can the struct outlive the data it borrows?
// A: No — that's exactly what the lifetime forbids; the compiler rejects it at compile time.
