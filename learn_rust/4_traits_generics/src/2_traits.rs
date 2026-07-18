// Q: Rust has no inheritance — how do types share behaviour? What's a default
//    method?

trait Summary {
    fn author(&self) -> String;
    fn summarize(&self) -> String {
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
    }
}

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn main() {
    notify(&Tweet { user: "amar".into() });
    notify(&Article { headline: "Rust wins".into() });
}

// A: Behaviour is shared through traits (a contract of methods) plus composition,
//    never class inheritance. A trait can supply a DEFAULT method body: Tweet uses the
//    default summarize(), Article overrides it.
//
// ── more Q&A ──
// Q: `&impl Summary` param vs a generic `<T: Summary>`?
// A: Same thing. Use the explicit generic when you must NAME T — e.g. return it, or
//    force two params to be the same type.
// Q: Can you implement an external trait on an external type?
// A: No — the orphan rule requires either the trait or the type to be local to your
//    crate, so impls can't collide across crates.
