// Q: str vs &str vs String? And why can't `let s: str = ...` live on the stack?

use std::fmt::Display;

fn print_it(s: &str) {
    println!("{s}");
}

fn main() {
    let owned: String = String::from("hello");
    let borrowed: &str = &owned;
    let slice: &str = &owned[1..3];
    println!("{owned} {borrowed} {slice}");

    print_it("literal");
    print_it(&owned);

    let d: &dyn Display = &42;
    let b: Box<dyn Display> = Box::new(42);
    println!("{d} {b}");
}

// A: String = (ptr,len,cap) owning a growable heap buffer. &str = a fat pointer
//    (ptr,len) borrowing UTF-8 it doesn't own, immutable. str is the bare text — a
//    Dynamically Sized Type whose size is unknown at compile time, so it can't sit on
//    the stack; it only exists behind a pointer (&str, Box<str>).
//
// ── more Q&A ──
// Q: What are the three DSTs?
// A: str, [T] (slice), and dyn Trait — all unsized, all used behind a pointer.
// Q: Why take `&str` instead of `&String` as a parameter?
// A: &str accepts more callers: string literals, &String (via Deref coercion), and
//    slices. &String only accepts a String.
// Q: Memory layout of &str vs String?
// A: &str = (ptr, len) — 2 words. String = (ptr, len, cap) — 3 words, owns the heap.
