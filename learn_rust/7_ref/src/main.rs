// Q: `&s`, `s.as_ref()`, `s.borrow()`, and `s.as_ptr()` can all point at the
//    same String. Are they the same tool with four spellings?

use std::borrow::Borrow;
use std::collections::HashMap;

fn print_text(label: &str, text: &str) {
    println!("{label}: '{text}' ({} bytes)", text.len());
}

fn accepts_many<T: AsRef<str>>(value: T) {
    let view: &str = value.as_ref();
    println!("AsRef view: {view}");
}

fn main() {
    let s = String::from("rust");

    let exact_ref: &String = &s; // borrow the exact String value
    print_text("&String coerces to &str", exact_ref);

    let as_ref_view: &str = s.as_ref(); // explicit cheap view conversion
    print_text("as_ref", as_ref_view);

    let borrow_view: &str = s.borrow(); // stronger contract than AsRef
    print_text("borrow", borrow_view);

    accepts_many("literal");
    accepts_many(&s);
    accepts_many(String::from("owned temp"));

    let mut scores = HashMap::new();
    scores.insert(String::from("rust"), 10);

    // HashMap<String, _> can be looked up with &str because String: Borrow<str>.
    println!("score = {}", scores.get("rust").unwrap());

    let ptr: *const u8 = s.as_ptr(); // raw address only: no length, no borrow safety
    println!("raw ptr = {ptr:p}, len = {}", s.len());

    // unsafe because YOU promise the pointer is valid and in bounds.
    unsafe {
        println!("first byte = {}", *ptr);
    }
}

// A: No. `&s` is a normal borrow of the exact value. Coercion may then turn `&String`
//    into `&str` at a call site. `.as_ref()` is an explicit generic "give me a cheap
//    shared view" conversion. `.borrow()` is for owned/borrowed forms that must compare
//    and hash the same way (why HashMap<String, _>.get("key") works). `.as_ptr()` gives
//    a raw pointer and drops the compiler's reference guarantees.
//
// ── more Q&A ──
// Q: Should I use `AsRef` or `Borrow` in my own function?
// A: Usually `AsRef`. Use `Borrow` when equality/hash/order of owned and borrowed forms
//    must match, like collection lookup keys.
// Q: Is `as_ptr()` just a faster reference?
// A: No. A raw pointer has no lifetime and no length. Dereferencing it is `unsafe`
//    because Rust cannot prove it still points to valid memory.
