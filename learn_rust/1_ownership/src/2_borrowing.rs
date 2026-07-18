// Q: How many &T and &mut T can be alive at once, and when does a borrow end?

fn main() {
    let mut s = String::from("hi");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;
    r3.push_str(" there");
    println!("{r3}");

    // let a = &s;
    // let b = &mut s;   // E0502: mutable borrow while immutably borrowed
    // println!("{a} {b}");
}

// A: Either any number of shared &T, OR exactly one exclusive &mut T — never both
//    at the same time. A borrow ends at its LAST USE, not the closing `}` (NLL),
//    which is why taking &mut s after r1/r2 are done compiles fine here.
//
// ── more Q&A ──
// Q: Why is only one &mut allowed at a time?
// A: Two mutable references could alias and mutate the same data simultaneously —
//    a data race. Exclusivity is what makes safe mutation sound.
// Q: Can you mutate through a shared &T?
// A: No. You need both a `mut` binding AND a `&mut` borrow to mutate.
// Q: Is the borrow checker a runtime or compile-time check?
// A: Compile time. RefCell is the tool that moves the same check to runtime.
// Q: Will this compile — what error, and how do you fix it?
//        let mut s = String::from("hi");
//        let r1 = &s;
//        let r2 = &mut s;
//        println!("{r1} {r2}");
//    A: No — E0502: cannot borrow `s` as mutable while it's borrowed as immutable (r1 is
//    still used in the println). Fix: finish using r1 BEFORE taking &mut s, so the shared
//    borrow ends (NLL) — e.g. `println!("{r1}");` first, then `let r2 = &mut s;`.
