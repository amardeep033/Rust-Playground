// Q: Predict — `first_word` needs NO lifetime annotation, but `pick` (same shape of body)
//    won't compile without one. What's different?

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap() // ✅ one input ref → output borrows it (elision rule 2)
}

// fn pick(a: &str, b: &str) -> &str {
//     a // ❌ E0106: TWO input refs — compiler can't guess which one the output ties to
// }

struct Parser<'a> {
    input: &'a str,
}
impl<'a> Parser<'a> {
    fn rest(&self) -> &str {
        self.input // ✅ &self present → output borrows self (elision rule 3), no 'a needed
    }
}

fn main() {
    println!("{}", first_word("hello world"));
    println!("{}", Parser { input: "config data" }.rest());
}

// A: Elision rules fill in lifetimes for common shapes: (2) exactly ONE input ref → its
//    lifetime goes to the output, so `first_word` is unambiguous. `pick` has TWO input
//    refs and no `&self`, so the compiler can't decide which the output ties to → you
//    must write `<'a>`. (3) `&self` always wins, which is why methods rarely need `'a`.
//
// ── more Q&A ──
// Q: So how do I quickly tell if a signature needs an explicit lifetime?
// A: Count input references that could be the output's source. One (or a `&self`) → elided.
//    Two or more with a borrowed return → you annotate. Owned return (`-> String`) → never.
// Q: Is elision "magic" or just a shorthand?
// A: Pure shorthand — the compiler expands it to the fully-annotated form. `first_word`
//    literally becomes `fn first_word<'a>(s: &'a str) -> &'a str`. Nothing is inferred at
//    runtime.
