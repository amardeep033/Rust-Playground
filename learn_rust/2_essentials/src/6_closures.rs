// Q: What are Fn, FnMut, and FnOnce, and when do you need a `move` closure?

fn main() {
    let factor = 3;
    let multiply = |x: i32| x * factor; // Fn: borrows `factor` immutably
    println!("{}", multiply(5));

    let mut count = 0;
    let mut inc = || count += 1; // FnMut: borrows `count` mutably
    inc();
    inc();
    println!("{count}");

    let owned = String::from("Amar");
    let consume = move || owned; // FnOnce + move: takes ownership, callable once
    println!("{}", consume());

    let data = vec![1, 2, 3];
    std::thread::spawn(move || println!("{data:?}")).join().unwrap();
}

// A: A closure captures its environment by the weakest access it needs — Fn
//    (immutable borrow), FnMut (mutable borrow), or FnOnce (takes ownership, callable
//    once). `move` forces capture BY VALUE.
//
// ── more Q&A ──
// Q: Why does thread::spawn / tokio::spawn require `move`?
// A: The closure may outlive the current stack frame; a borrow of a local would
//    dangle, so it must OWN what it captures.
// Q: Which trait does `|x| x * factor` implement here?
// A: Fn — it only reads the captured `factor`.
// Q: Can a function return a closure?
// A: Yes: `-> impl Fn(..)` for one concrete closure, or `Box<dyn Fn(..)>` to return
//    different closures.
