// Q: Count how many times each word appears — without doing two lookups per key.

use std::collections::HashMap;

fn main() {
    let words = ["apple", "banana", "apple", "cherry", "banana", "apple"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in words {
        *counts.entry(w).or_insert(0) += 1;
    }
    println!("{counts:?}");

    let mut scores: HashMap<&str, i32> = HashMap::new();
    for name in ["a", "b", "a"] {
        scores.entry(name).and_modify(|s| *s += 10).or_insert(1);
    }
    println!("{scores:?}");
}

// A: `entry(k).or_insert(0)` returns a &mut to the value, inserting the default only
//    when the key is absent — one lookup for get-or-create, then `*.. += 1`. When the
//    update differs from the initial value, use `.and_modify(..).or_insert(default)`.
//
// ── more Q&A ──
// Q: or_insert vs or_insert_with?
// A: or_insert takes an already-built value; or_insert_with takes a closure and only
//    builds the default when needed — cheaper if the default allocates.
// Q: Is HashMap iteration ordered?
// A: No, order is arbitrary. Use BTreeMap when you need keys in sorted order.
// Q: What does get() return?
// A: Option<&V> — None if the key is absent, so no panic.
