// Q: Predict — `Tweet` implements `author` but NOT `summarize`, yet `tweet.summarize()`
//    compiles and runs. How?

trait Summary {
    fn author(&self) -> String;
    fn summarize(&self) -> String {
        // default body
        format!("(by {})", self.author())
    }
}

struct Tweet {
    user: String,
}
impl Summary for Tweet {
    fn author(&self) -> String {
        self.user.clone()
    }
    // no summarize → uses the default
}

struct Article {
    headline: String,
}
impl Summary for Article {
    fn author(&self) -> String {
        "staff".into()
    }
    fn summarize(&self) -> String {
        format!("{} — {}", self.headline, self.author())
    } // overrides
}

fn main() {
    println!("{}", Tweet { user: "amar".into() }.summarize()); // (by amar)   — default
    println!("{}", Article { headline: "Rust wins".into() }.summarize()); // Rust wins — staff — override
}

// A: `summarize` has a DEFAULT implementation in the trait, so any implementor gets it for
//    free unless it overrides it. Tweet inherits the default; Article overrides. Default
//    methods can even call other trait methods (`self.author()`) that each type supplies —
//    template-method pattern without inheritance.
//
// ── more Q&A ──
// Q: Can you `impl Display for Vec<i32>` in your crate?
// A: No — the orphan rule: you can only implement a trait if the trait OR the type is local
//    to your crate. Both `Display` and `Vec` are foreign. Wrap it in a newtype (`struct
//    MyVec(Vec<i32>)`) and impl on that.
// Q: `fn f(x: &impl Summary)` vs `fn f<T: Summary>(x: &T)` — same or different?
// A: Same generated code. Use the explicit `<T>` when you must NAME the type — e.g. return
//    `T`, or require two parameters to be the SAME type. `impl Trait` can't express that.
