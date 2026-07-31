// Q: `&mut nums`, `nums.as_mut()`, `nums.borrow_mut()`, and `nums.as_mut_ptr()`
//    can all mutate the same Vec. Which one still lets Rust protect you?

use std::borrow::BorrowMut;

fn bump_first(values: &mut [i32]) {
    values[0] += 1;
}

fn main() {
    let mut nums = vec![10, 20, 30];

    {
        let exact: &mut Vec<i32> = &mut nums; // exact mutable borrow of the Vec
        exact.push(40);
    } // exact borrow ends here

    let slice_view: &mut [i32] = nums.as_mut(); // Vec<i32> -> &mut [i32]
    bump_first(slice_view);

    let borrowed_view: &mut [i32] = nums.borrow_mut();
    borrowed_view[1] += 2;

    println!("after safe mutation: {nums:?}");

    let ptr: *mut i32 = nums.as_mut_ptr();

    // Raw pointer mutation is possible, but Rust cannot check aliasing or bounds here.
    unsafe {
        *ptr.add(2) += 3;
    }

    println!("after raw pointer mutation: {nums:?}");

    // let r = &nums[0];
    // let m = &mut nums;
    // println!("{r} {m:?}"); // ❌ E0502: shared and mutable borrows overlap
}

// A: The first three are safe borrows/views, so Rust still enforces "one mutable OR many
//    shared". `&mut nums` borrows the Vec itself. `.as_mut()` is a generic mutable view
//    conversion, commonly Vec -> slice. `.borrow_mut()` is the BorrowMut version. But
//    `.as_mut_ptr()` gives a raw pointer; using it moves the aliasing/bounds promise into
//    your `unsafe` block.
//
// ── more Q&A ──
// Q: Why did the `exact` borrow need its own block?
// A: To make the lesson obvious. NLL would often end it at last use anyway, but the block
//    makes the handoff from `&mut Vec<i32>` to `&mut [i32]` visible.
// Q: Is `as_mut_ptr()` bad?
// A: No. It's necessary for FFI and low-level buffer work. The trick is keeping the
//    unsafe region tiny and proving no other reference observes invalid mutation.
