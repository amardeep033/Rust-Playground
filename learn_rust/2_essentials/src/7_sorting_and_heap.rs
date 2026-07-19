// Q: You need the 3 LARGEST of a million numbers. Predict — is sorting all million and
//    taking the last 3 the right move, or is there something cheaper?

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // sort by a field (ascending), then a full comparator (descending)
    let mut people = vec![("Amar", 30), ("Deep", 25), ("John", 35)];
    people.sort_by_key(|&(_, age)| age);
    people.sort_by(|a, b| b.1.cmp(&a.1)); // descending: note b.cmp(a)
    println!("{people:?}");

    // top-K with a size-k MIN-heap — O(n log k), never sorts the whole input
    let nums = [5, 1, 9, 3, 7, 8, 2];
    let k = 3;
    let mut heap = BinaryHeap::new(); // BinaryHeap is a MAX-heap...
    for &n in &nums {
        heap.push(Reverse(n)); // ...so Reverse makes it behave as a MIN-heap
        if heap.len() > k {
            heap.pop(); // drop the smallest so far → heap keeps the k largest
        }
    }
    let mut top: Vec<i32> = heap.into_iter().map(|Reverse(n)| n).collect();
    top.sort_unstable_by(|a, b| b.cmp(a));
    println!("top {k}: {top:?}"); // [9, 8, 7]
}

// A: Sorting all n is O(n log n) and wasteful for k ≪ n. Keep a size-k heap and pop the
//    smallest whenever it grows past k → O(n log k), constant extra memory. The catch:
//    `BinaryHeap` is a MAX-heap, so to keep the LARGEST you need a MIN-heap — wrap values
//    in `std::cmp::Reverse` to flip the ordering. (Forgetting Reverse is the classic slip.)
//
// ── more Q&A ──
// Q: sort() vs sort_unstable() — which to reach for?
// A: `sort_unstable` is faster and allocation-free but may reorder equal elements. `sort`
//    is stable (keeps equal elements' original order) but allocates. Default to unstable
//    unless you rely on stability.
// Q: How would you merge k already-sorted streams?
// A: Same heap trick: push the head of each stream, pop the smallest, then push that
//    stream's next element. That's a k-way merge in O(total log k).
