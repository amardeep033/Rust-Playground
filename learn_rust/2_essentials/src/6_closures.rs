// Q: Predict — this closure is passed to a thread. Without the `move` keyword, does it
//    compile?

use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // let h = thread::spawn(|| println!("{data:?}")); // ❌ closure borrows `data`, which may outlive main's frame
    let h = thread::spawn(move || println!("{data:?}")); // ✅ move: closure OWNS `data`
    h.join().unwrap();

    // The three closure traits, by how they capture:
    let factor = 3;
    let mul = |x: i32| x * factor; // Fn    — reads `factor`
    println!("{}", mul(5));

    let mut n = 0;
    let mut inc = || n += 1; // FnMut  — mutates `n`
    inc();
    inc();
    println!("{n}");

    let s = String::from("owned");
    let consume = move || s; // FnOnce — moves `s` out, callable ONCE
    println!("{}", consume());
    // consume(); // ❌ can't call again — it gave `s` away the first time
}

// A: Without `move` it does NOT compile: a bare closure borrows `data`, but the thread may
//    outlive the current stack frame, so that borrow could dangle. `move` forces capture
//    BY VALUE, giving the thread ownership. A closure captures by the weakest access it
//    needs — Fn (&), FnMut (&mut), FnOnce (owns) — and `move` just changes borrow → own.
//
// ── more Q&A ──
// Q: Why is `consume` callable only once?
// A: It returns (moves out) its captured `s`. After the first call `s` is gone, so calling
//    again would use a moved value → it's `FnOnce`. Closures that consume captures are one-shot.
// Q: Can a function return a closure?
// A: Yes: `-> impl Fn(i32) -> i32` for one concrete closure (zero-cost), or `Box<dyn Fn(..)>`
//    when you need to return different closures from different branches.
