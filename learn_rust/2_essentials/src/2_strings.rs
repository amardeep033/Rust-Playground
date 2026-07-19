// Q: Predict — does `let c = word[0];` give you the first character? (word = "héllo")

fn main() {
    let word = "héllo";
    // let c = word[0]; // ❌ won't compile — String/&str do NOT implement indexing by usize
    println!("first byte : {}", word.as_bytes()[0]); // a byte (may be half a char!)
    println!("first char : {}", word.chars().next().unwrap()); // 'h' — a real Unicode char

    // The reason indexing is banned: bytes ≠ chars.
    println!("bytes={} chars={}", word.len(), word.chars().count()); // 6 bytes, 5 chars ('é' = 2 bytes)

    // Practical: parse `key = value`
    if let Some((k, v)) = "  name = Amar  ".trim().split_once('=') {
        println!("k='{}' v='{}'", k.trim(), v.trim());
    }
}

// A: No — `word[0]` doesn't even COMPILE. Rust forbids `str` indexing by position because
//    UTF-8 chars are variable-width: `word[1]` could land in the MIDDLE of 'é' and give
//    you half a character. Use `.chars()` for characters, `.as_bytes()`/`.bytes()` for
//    bytes, or slice by a known-valid range (`&word[0..1]`, which panics if it splits a char).
//
// ── more Q&A ──
// Q: `word.len()` returned 6 for a 5-char string — bug?
// A: No — `.len()` is the BYTE length, not the character count. 'é' is 2 UTF-8 bytes. Use
//    `.chars().count()` for characters (it's O(n), because of that variable width).
// Q: split vs split_once — when each?
// A: `split` yields ALL pieces (an iterator); `split_once` cuts at the FIRST delimiter and
//    returns `Option<(&str, &str)>` — exactly what you want for `key=value` where the
//    value itself might contain `=`.
