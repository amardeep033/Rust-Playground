// Q: Predict — to count word frequencies, why can't you just write `counts[w] += 1`?
//    What does `entry(w).or_insert(0)` do that indexing can't?

use std::collections::HashMap;

fn main() {
    let words = ["apple", "banana", "apple", "cherry", "banana", "apple"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in words {
        // counts[w] += 1;              // ❌ won't compile — HashMap has no IndexMut, and the key may be absent
        *counts.entry(w).or_insert(0) += 1; // ✅ insert 0 if missing, hand back &mut, then increment
    }
    println!("{counts:?}"); // {"apple": 3, ...}

    // Reading a missing key:
    // println!("{}", counts["grape"]);        // ❌ PANICS — key not found
    println!("{}", counts.get("grape").copied().unwrap_or(0)); // ✅ 0
}

// A: `counts[w] += 1` doesn't compile: HashMap intentionally has no mutable indexing, and
//    even reading a missing key with `counts["grape"]` PANICS. `entry(w).or_insert(0)`
//    handles the "maybe absent" case in ONE lookup — insert the default if missing, then
//    return a `&mut` you can update. It's the idiom precisely because indexing is unsafe here.
//
// ── more Q&A ──
// Q: or_insert vs or_insert_with — when does the difference matter?
// A: `or_insert(expensive())` builds the default EVERY call even when the key exists.
//    `or_insert_with(|| expensive())` only builds it when actually inserting — use it if
//    the default allocates or computes.
// Q: You iterate the map and the order changes each run — bug?
// A: No — HashMap order is intentionally randomized (DoS resistance). Need sorted/stable
//    order? Use `BTreeMap`, or collect keys into a Vec and `sort()`.
