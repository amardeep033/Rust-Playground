// Q: Predict — `make_ref` and `make_owned` look almost identical. One compiles, one
//    doesn't. Which, and why?

// fn make_ref() -> &String {
//     let s = String::from("hello");
//     &s // ❌ E0106: "missing lifetime — there's nothing for the return to borrow from"
// }        //    (s is a local; it would be dropped, so the return can't borrow anything valid)

fn make_owned() -> String {
    let s = String::from("hello");
    s // ✅ ownership MOVES out to the caller — nothing is dropped
}

fn main() {
    let a = String::from("hello");
    let r = &a;
    println!("{r}"); // r's last use → borrow ends
    let b = a; // ✅ move is fine now
    println!("{b}");
    // If `println!("{r}")` came AFTER `let b = a;` → ❌ E0505: can't move `a` while borrowed

    println!("{}", make_owned());
}

// A: `make_ref` fails: `s` is a local, dropped the instant the function returns, so `&s`
//    would dangle. `make_owned` returns the value itself — ownership transfers to the
//    caller, so there's nothing to dangle. The fix for "can't return a reference" is
//    almost always "return the owned value".
//
// ── more Q&A ──
// Q: But real functions DO return `&str`/`&T` all the time — how, if not from a local?
// A: They return a reference derived from an INPUT (`fn first(v: &[i32]) -> &i32`). The
//    caller owns the data and lent it in; the output borrow ties to that input's lifetime,
//    so it can't outlive it. You never return a ref to something the function itself owns.
// Q: Dangling pointer vs use-after-free — same thing?
// A: Related, not identical. A dangling pointer is an invalid reference that EXISTS (a
//    state); use-after-free is DEREFERENCING it (an action). Rust kills the bug at the
//    first stage — the dangling ref never even compiles.
