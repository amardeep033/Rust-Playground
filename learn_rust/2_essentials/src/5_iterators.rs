// Q: Iterators are "lazy" — what does that mean in practice? Build a pipeline that
//    keeps evens, doubles them, and collects.

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let evens_doubled: Vec<i32> =
        nums.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
    println!("{evens_doubled:?}");

    let raw = ["1", "two", "3", "x", "5"];
    let parsed: Vec<i32> = raw.iter().filter_map(|s| s.parse().ok()).collect();
    println!("{parsed:?}");

    let picked: Vec<i32> = (10..).skip(2).take(3).collect();
    println!("{picked:?}");
}

// A: Adapters (map/filter/filter_map/take/skip) build a pipeline but do NOTHING
//    until a consumer (collect/sum/count/for) pulls values through it — which is why
//    `(10..)` can be an infinite range yet `.skip(2).take(3)` is fine.
//
// ── more Q&A ──
// Q: filter_map vs filter then map?
// A: filter_map does transform-and-drop-invalid in ONE pass: the closure returns
//    Option, and None values are dropped (great for skipping bad rows via `.ok()`).
// Q: What actually drives an iterator to run?
// A: A consuming/terminal method — collect, sum, count, for, fold. Adapters alone are lazy.
// Q: fold vs reduce?
// A: fold takes an explicit initial accumulator; reduce uses the first element as the
//    seed and returns Option (None on an empty iterator).
