// Q: The BODY of `longest` is obviously fine — it just returns x or y. So why does the
//    compiler reject it without the `<'a>` annotation?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// Without <'a>: ❌ E0106 missing lifetime specifier — the body isn't the problem, the
// CALLER is: the compiler can't tell how long the returned reference stays valid.

fn main() {
    let s1 = String::from("long string");
    println!("{}", longest(&s1, "short")); // ✅

    // The trap the annotation actually guards against:
    // let result;
    // {
    //     let tmp = String::from("temporary");
    //     result = longest(&s1, &tmp); // result may borrow tmp
    // } // tmp dropped here
    // println!("{result}"); // ❌ tmp doesn't live long enough — 'a = the SHORTER of the two
}

// A: The compiler analyses each function in ISOLATION. It doesn't see the body's outcome;
//    it sees the signature: "returns a ref that could be x or y." Without `'a` it can't
//    encode "the result lives only as long as the shorter input", so a caller could hold
//    the result past an input's death → dangling. `<'a>` writes that contract down.
//
// ── more Q&A ──
// Q: Do lifetimes make the data live longer?
// A: No — they only NAME and relate existing lifetimes. They constrain what you can do;
//    they never extend how long `tmp` actually lives. That's the #1 misconception.
// Q: In real code, how often do you write explicit `'a`?
// A: Rarely in function bodies — elision handles most (next file). You mostly write them
//    on structs that hold references, and occasionally on multi-input functions like this.
