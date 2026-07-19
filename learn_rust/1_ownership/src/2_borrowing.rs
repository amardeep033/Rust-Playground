// Q: This compiles. Move the `println!("{r1}")` to the BOTTOM and it won't. Why would
//    the same three lines suddenly fail?

fn main() {
    let mut s = String::from("hi");

    let r1 = &s; // shared borrow
    println!("{r1}"); // ← r1's LAST use is here; the shared borrow ends now (NLL)

    let r2 = &mut s; // ✅ allowed, because r1 is already done
    r2.push_str(" there");
    println!("{r2}");

    // If instead you wrote:
    //     let r1 = &s;
    //     let r2 = &mut s;
    //     println!("{r1} {r2}");   // ❌ E0502: r1 is still alive HERE, overlapping &mut
}

// A: The borrow checker cares about the LAST USE of a borrow, not where it was declared
//    (non-lexical lifetimes). In the working version r1 is finished before `&mut s` is
//    taken, so shared and mutable never overlap. Move r1's use after the `&mut` and they
//    overlap → E0502. Same lines, different lifetimes.
//
// ── more Q&A ──
// Q: Why does Rust forbid a `&` and `&mut` at the same time at all — isn't reading harmless?
// A: If code holds `&s` (assuming s won't change) while another path holds `&mut s`, a
//    mutation could invalidate the reader (e.g. a Vec reallocating moves its buffer,
//    dangling the `&`). Exclusive-xor-shared is what makes that impossible.
// Q: Is this a real safety win or just the compiler being strict?
// A: Real: it's the compile-time version of what causes iterator-invalidation and data-
//    race bugs in C++/Java. You trade a little friction for "these bugs can't exist".
