// Q: `fn longest(x: &str, y: &str) -> &str` won't compile. Why, and how do you
//    fix it?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("short");
    println!("{}", longest(&s1, &s2));
}

// A: The return is either x or y, which may have different lifetimes, so the
//    compiler can't tell how long the result is valid (E0106). Add a shared lifetime
//    <'a> tying the output to both inputs — the result is valid for the SHORTER of
//    the two.
//
// ── more Q&A ──
// Q: Do lifetimes change how long data actually lives?
// A: No. They only NAME and relate existing lifetimes so the compiler can reject
//    dangling references — they constrain, they don't extend.
// Q: What does `'a` mean in `longest<'a>`?
// A: "The returned reference is valid for at least as long as the shorter of x and y."
