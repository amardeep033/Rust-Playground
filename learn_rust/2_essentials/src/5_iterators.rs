// Q: Predict what this prints:  `(1..=3).map(|x| println!("side effect {x}"));`
//    Three lines? Zero lines?

fn main() {
    (1..=3).map(|x| println!("side effect {x}")); // ← prints NOTHING (and warns: unused iterator)

    // Add a consumer and it runs:
    (1..=3).for_each(|x| println!("with for_each {x}")); // prints 1,2,3

    // Real pipeline — still lazy until `collect` pulls it:
    let evens_doubled: Vec<i32> = (1..=6)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
        .collect(); // ← THIS is what actually drives the chain
    println!("{evens_doubled:?}"); // [4, 8, 12]

    // filter_map = transform + drop-invalid in one pass
    let parsed: Vec<i32> = ["1", "two", "3"].iter().filter_map(|s| s.parse().ok()).collect();
    println!("{parsed:?}"); // [1, 3]
}

// A: The `.map(...)` line prints NOTHING. Iterator adapters (map/filter/…) are LAZY — they
//    build a recipe and do zero work until a CONSUMER (for/collect/sum/count/for_each)
//    pulls values through. This is the classic "my map didn't run" bug. `.collect()`,
//    `.for_each()`, or a `for` loop are what actually execute the chain.
//
// ── more Q&A ──
// Q: How do I know a method is a lazy adapter vs an eager consumer?
// A: Adapters return another iterator (map, filter, take, zip); consumers return a value or
//    `()` (collect, sum, count, for_each, find, any). If it returns an iterator, nothing ran yet.
// Q: filter_map vs filter().map() — why prefer it?
// A: `filter_map` does "convert, and skip the ones that fail" in a single pass (the closure
//    returns `Option`, `None`s are dropped) — perfect for "parse these lines, ignore bad rows".
