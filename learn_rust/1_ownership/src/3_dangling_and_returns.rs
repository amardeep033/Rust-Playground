// Q: Why can't a function return `&String` to a local, and why can't you move a
//    value while it's borrowed?

fn make() -> String {
    let s = String::from("hello");
    s
}

// fn bad() -> &String {
//     let s = String::from("hello");
//     &s // E0515: returns a reference to data owned by this function
// }

fn main() {
    let a = String::from("hello");
    let r = &a;
    println!("{r}");
    let b = a;
    println!("{b}");

    println!("{}", make());
}

// A: A reference must never outlive the data it points to. A local is dropped when
//    the function returns, so returning `&local` would dangle — return the owned
//    String instead. Likewise moving `a` while `r` still borrows it would leave `r`
//    dangling (E0505); it's allowed here only because `r`'s last use (NLL) is before
//    the move.
//
// ── more Q&A ──
// Q: What's the difference between a dangling pointer and a use-after-free?
// A: A dangling pointer is an invalid reference that EXISTS (a state); use-after-free
//    is DEREFERENCING it (an action). Every use-after-free involves a dangling
//    pointer, but a dangling pointer isn't a bug until it's used.
// Q: How can a function return borrowed data safely?
// A: The caller owns the data and lends a reference in (with a lifetime tying output
//    to input), or the function returns owned data (String).
