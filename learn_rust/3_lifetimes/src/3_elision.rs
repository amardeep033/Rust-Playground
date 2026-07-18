// Q: When can you omit lifetime annotations entirely? Name the rules.

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap()
}

struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn rest(&self) -> &str {
        self.input
    }
}

fn combine(a: &str, b: &str) -> String {
    format!("{a}{b}")
}

fn main() {
    println!("{}", first_word("hello world"));
    println!("{}", Parser { input: "config data" }.rest());
    println!("{}", combine("foo", "bar"));
}

// A: Three elision rules: (1) each &param gets its own lifetime; (2) with exactly one
//    input lifetime it's used for all outputs (so first_word needs none); (3) if
//    &self/&mut self is present, its lifetime goes to the outputs (so rest() needs none).
//
// ── more Q&A ──
// Q: Why does one input ref elide but two don't?
// A: Rule 2 only applies with a SINGLE input lifetime. With two, the compiler can't
//    guess which one the output ties to → you must annotate.
// Q: What is `'static`?
// A: A lifetime lasting the whole program (e.g. string literals). It's a real bound,
//    not a fix-all — slapping it on to silence an error usually just hides the real one.
