// Q: Predict — after the loop, can you still use `nums`? It depends on ONE character in
//    `for x in nums` vs `for x in &nums`. Which lets you use `nums` afterward?

fn main() {
    let nums = vec![10, 20, 30];

    for x in &nums {
        // &nums → iter() → &i32, borrows
        print!("{x} ");
    }
    println!("→ still usable: {nums:?}"); // ✅ nums borrowed, not consumed

    let total: i32 = nums.into_iter().sum(); // nums → into_iter() → i32, MOVES/consumes
    println!("sum = {total}");
    // println!("{nums:?}"); // ❌ E0382: nums was consumed by into_iter()

    // dedup gotcha:
    let mut v = vec![1, 3, 1, 1, 2];
    v.dedup(); // [1, 3, 1, 2] — only removes CONSECUTIVE dups!
    println!("{v:?}");
    v.sort();
    v.dedup(); // now [1, 2, 3]
    println!("{v:?}");
}

// A: `for x in &nums` borrows (iter), so `nums` survives the loop. `for x in nums` moves
//    (into_iter), CONSUMING it — use it afterward and you get E0382. The plain `for x in
//    nums` reads innocent but silently takes ownership; that surprises people who then
//    can't touch the Vec again.
//
// ── more Q&A ──
// Q: Why did the first `dedup()` leave duplicates in?
// A: `dedup` only collapses ADJACENT equal elements — it's O(n) and assumes sorted input.
//    On unsorted data you must `sort()` first, or it silently keeps non-adjacent dups.
// Q: `Vec::new()` vs `Vec::with_capacity(n)` — does it matter?
// A: Functionally no; performance yes. `with_capacity` pre-allocates so pushing n items
//    doesn't repeatedly reallocate + copy the buffer. Use it when you know the rough size.
