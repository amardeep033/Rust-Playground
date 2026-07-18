// Q: What happens on integer overflow? And how do tuples, arrays, and slices
//    differ?

fn main() {
    let x: u8 = 255;
    // let y = x + 1;                    // debug: PANICS "overflow"; release: wraps to 0
    println!("wrapping:   {}", x.wrapping_add(1)); // 0
    println!("checked:    {:?}", x.checked_add(1)); // None
    println!("saturating: {}", x.saturating_add(1)); // 255

    let t: (i32, &str, bool) = (1, "hi", true);
    let (a, b, c) = t;
    println!("{a} {b} {c} / {}", t.0);

    let arr: [i32; 3] = [10, 20, 30];
    let s: &[i32] = &arr[1..];
    println!("{arr:?} slice {s:?}");
}

// A: In debug builds integer overflow PANICS; in release it wraps (two's
//    complement) — be explicit with wrapping_/checked_/saturating_add. A tuple is
//    a fixed-size group of MIXED types (index `.0` or destructure). An array
//    `[T; N]` is a fixed-size, SAME-type block on the stack. A slice `&[T]` is a
//    borrowed view (ptr+len) into an array/Vec.
//
// ── more Q&A ──
// Q: What is usize for?
// A: A pointer-sized unsigned integer used for indexing and lengths (e.g. Vec::len).
// Q: How big is a char, and what does it hold?
// A: 4 bytes — a Unicode scalar value, not a single byte.
// Q: Is a slice `[T]` Sized?
// A: No, `[T]` is a DST (dynamically sized); you always use it behind a pointer, `&[T]`.
