// Q: How do you sort structs by a field, and get the top-K largest without a full
//    sort?

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut people = vec![("Amar", 30), ("Deep", 25), ("John", 35)];
    people.sort_by_key(|&(_, age)| age); // ascending by age
    println!("{people:?}");
    people.sort_by(|a, b| b.1.cmp(&a.1)); // descending by age
    println!("{people:?}");

    let nums = [5, 1, 9, 3, 7, 8, 2];
    let k = 3;
    let mut heap = BinaryHeap::new();
    for &n in &nums {
        heap.push(Reverse(n));
        if heap.len() > k {
            heap.pop(); // drop the smallest kept so far
        }
    }
    let mut top: Vec<i32> = heap.into_iter().map(|Reverse(n)| n).collect();
    top.sort_unstable_by(|a, b| b.cmp(a));
    println!("top {k}: {top:?}"); // [9, 8, 7]
}

// A: `sort_by_key` sorts by a derived key; `sort_by` takes a full comparator
//    (`b.cmp(&a)` for descending). For top-K, keep a size-k MIN-heap: BinaryHeap is a
//    MAX-heap, so wrap values in std::cmp::Reverse to invert it, push all n items, and
//    pop whenever len > k — O(n log k), no full sort.
//
// ── more Q&A ──
// Q: sort vs sort_unstable?
// A: sort is stable (keeps the order of equal elements) but allocates; sort_unstable
//    is faster and allocation-free but may reorder equal elements.
// Q: Is BinaryHeap a max-heap or min-heap?
// A: Max-heap (pop() gives the largest). Wrap items in std::cmp::Reverse to get a min-heap.
// Q: How would you merge k sorted streams?
// A: Same heap idea: push the head of each stream, pop the smallest, then push that
//    stream's next element.
