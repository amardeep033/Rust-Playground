// Q: Raw pointers can be copied freely and mutated through `unsafe`. Does that mean
//    they bypass Rust's rules safely?

fn main() {
    let mut data = vec![1, 2, 3, 4];

    let ptr = data.as_mut_ptr();
    let len = data.len();

    // Safe Rust cannot tell whether ptr is valid, in-bounds, aligned, or uniquely usable.
    // This block is the boundary where we must prove those facts ourselves.
    unsafe {
        for i in 0..len {
            *ptr.add(i) *= 10;
        }
    }

    println!("{data:?}");

    // The bug-shaped version:
    // let ptr = data.as_mut_ptr();
    // data.push(5);        // may reallocate, making ptr stale
    // unsafe { *ptr = 99 } // ❌ possible use-after-free / write through stale pointer
}

// A: Raw pointers do bypass compile-time borrow checking, but they don't make invalid
//    memory valid. They are how Rust talks to lower-level worlds: FFI, custom allocators,
//    intrusive data structures, and performance-critical buffers. The price is that an
//    `unsafe` block must re-establish the rules manually: valid pointer, correct alignment,
//    in bounds, initialized memory, and no illegal aliasing.
//
// ── more Q&A ──
// Q: Why store `len` beside the pointer?
// A: A raw pointer is just an address. Slices carry pointer + length; raw pointers don't.
// Q: Why is `push` after `as_mut_ptr()` dangerous?
// A: Vec may reallocate to grow. Reallocation moves the buffer, so an old pointer can point
//    to freed memory. The pointer still exists, but using it may be undefined behavior.
