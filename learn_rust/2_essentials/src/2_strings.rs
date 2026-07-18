// Q: Parse `key = value` out of a messy line like "  name = Amar  ".

fn main() {
    let line = "  name = Amar  ";

    if let Some((key, value)) = line.trim().split_once('=') {
        println!("key='{}' value='{}'", key.trim(), value.trim());
    }

    let csv = "10,20,30,40";
    let nums: Vec<i32> = csv.split(',').map(|s| s.parse().unwrap()).collect();
    println!("{nums:?}");
}

// A: trim() strips surrounding whitespace; split_once('=') cuts on the FIRST '='
//    into a (before, after) tuple, then trim each side. split(',') + parse() +
//    collect() turns a delimited string into a typed Vec.
//
// ── more Q&A ──
// Q: split vs split_once?
// A: split yields ALL pieces as an iterator; split_once cuts at the first delimiter
//    only and returns an Option<(&str, &str)>.
// Q: What does .parse() return, and how does it know the target type?
// A: Result<T, _>; the type comes from annotation or turbofish (`"42".parse::<i32>()`).
// Q: bytes vs chars, and can you index a String by position?
// A: .bytes() yields u8, .chars() yields Unicode scalars. You canNOT do `s[i]` — a
//    byte index could split a multi-byte char; slice by range or iterate chars.
