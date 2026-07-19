// Q: What happens when the data a struct borrows is dropped before the struct is used?

struct Person<'a> {
    id: u32,
    name: &'a str,
}

fn main() {
    let p;
    let p2;

    {
        let s = String::from("Alice");
        p = Person { id: 1, name: &s };
        p2 = Person {
            id: 2,
            name: "Amar", // `&` not needed; string literals are already `&'static str`
        };
    }

    // println!("{}", p.id); // ❌ Compile error: `s` (and so `p.name`) was dropped at the end of the block
    println!("{}", p2.name); // ✅ works — string literals are `&'static str`, so p2.name outlives the block
}

// A: The struct's lifetime `'a` ties it to whatever it borrows, so `p` is only valid as
//    long as `s` is alive. Once the block ends, `s` is dropped, `p.name` becomes a
//    dangling reference, and the compiler rejects any later use of `p` — the whole
//    struct is only as long-lived as its shortest-lived borrowed field.
//
// ── more Q&A ──
// Q: Why does `p2` still work after the block, but `p` doesn't?
// A: `p2.name` borrows `&"Amar"`, a string literal. Literals are baked into the binary
//    and promoted to `&'static str`, so they outlive every scope — `p2` is fine even
//    though it was also created inside the block.
// Q: When is storing `&str` in a struct (instead of an owned `String`) actually useful?
// A: When you want to avoid cloning — e.g. parsing a file into many structs that each
//    reference a slice of one owned buffer, instead of allocating a new String per struct.
