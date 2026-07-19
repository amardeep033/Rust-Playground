// Q: Predict — `255u8 + 1`. Does it (a) not compile, (b) panic, (c) give 0, or (d) give
//    256? And is the answer the SAME in `cargo run` and `cargo run --release`?

fn main() {
    let x: u8 = 255;
    // let y = x + 1; // ❗ debug: PANICS "attempt to add with overflow" · release: SILENTLY wraps to 0
    println!("wrapping:   {}", x.wrapping_add(1)); // 0    (explicit wrap)
    println!("checked:    {:?}", x.checked_add(1)); // None (explicit "did it overflow?")
    println!("saturating: {}", x.saturating_add(1)); // 255  (clamp to max)

    let t: (i32, &str, bool) = (1, "hi", true);
    let (a, b, c) = t; // destructure
    println!("{a} {b} {c} / {}", t.0);

    let arr = [10, 20, 30];
    let s: &[i32] = &arr[1..]; // slice = borrowed window, not a copy
    println!("{arr:?} {s:?}");
}

// A: (b) then (c) — it's the trap: `255u8 + 1` PANICS in debug but SILENTLY WRAPS to 0 in
//    release. Same code, different behaviour by build profile. That's why you never rely
//    on `+` for values that can overflow — say what you mean with
//    wrapping_/checked_/saturating_add. Tuple = fixed mixed types; array = fixed one type
//    on the stack; slice `&[T]` = a borrowed view (ptr+len), a DST used behind `&`.
//
// ── more Q&A ──
// Q: When do you actually reach for checked_/wrapping_/saturating_?
// A: checked_ when overflow is a real error (parsing, allocation sizes) → handle the None.
//    wrapping_ when you WANT modular arithmetic (hashing, ring buffers). saturating_ for
//    clamping (progress bars, ret/backoff counters that must not exceed a cap).
// Q: Why can't you index a slice safely with `s[99]`?
// A: You can index, but out-of-bounds `s[99]` PANICS. `s.get(99)` returns `Option<&T>` —
//    the safe form. Same story as `Vec`.
