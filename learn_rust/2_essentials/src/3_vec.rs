// Q: `for x in v`, `for x in &v`, `for x in &mut v` — what's the ownership
//    difference between the three?

fn main() {
    let mut v = Vec::with_capacity(4);
    v.push(3);
    v.push(1);
    v.push(2);
    v.push(2);

    v.sort();
    v.dedup();
    v.retain(|&x| x != 2);
    println!("{v:?}");

    let mut nums = vec![10, 20, 30];
    for x in &nums {
        print!("{x} ");
    }
    for x in &mut nums {
        *x += 1;
    }
    let total: i32 = nums.into_iter().sum();
    println!("\n{total}");
}

// A: `&v` calls iter() → yields &T, an immutable borrow (v still usable after).
//    `&mut v` calls iter_mut() → yields &mut T, mutating in place.
//    `v` calls into_iter() → yields T, consuming/moving v (unusable afterward).
//
// ── more Q&A ──
// Q: Vec::new vs Vec::with_capacity?
// A: with_capacity preallocates the buffer, avoiding repeated reallocation+copy as
//    it grows — use it when you know the rough size.
// Q: Why did dedup need sort() first?
// A: dedup only removes CONSECUTIVE duplicates; sorting groups equal values together.
// Q: Does indexing a Vec panic?
// A: `v[i]` panics on out-of-bounds; `v.get(i)` returns Option<&T> for safe access.
