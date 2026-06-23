# Types and Error Handling

## String Types

The most common interview topic for Rust beginners. Three types, three different roles.

| Type | Storage | Owned? | Mutable? | When to use |
|------|---------|--------|----------|-------------|
| `str` | anywhere (DST) | no | N/A | never directly — always via `&str` or `Box<str>` |
| `&str` | fat pointer on stack | no (borrows) | no | read-only text, function params, string literals |
| `String` | heap | yes | yes | owned, growable text |

### Memory Layout

`&str` is a fat pointer: `(ptr_to_utf8_bytes, length)`.  
`String` is: `(ptr_to_heap, length, capacity)`.

```rust
fn main() {
    let a: &'static str = "hello";        // in binary, 'static lifetime
    let b: String = String::from("hello"); // heap-allocated, owned
    let c: &str = &b;                     // borrow of b's contents
    let d: &str = &b[1..3];              // slice: "el"
    println!("{a} {b} {c} {d}");
}
```

### What happens?
```rust
let x: str = *"hello"; // ?
```
**Compile error**: `str` is a DST (dynamically sized type) — size unknown at compile time. Cannot live on the stack directly.

Always use `&str` or `String`.

### Conversion patterns
```rust
// &str → String
let s: String = "hello".to_string();
let s: String = String::from("hello");
let s: String = "hello".to_owned();

// String → &str
let owned = String::from("hi");
let borrowed: &str = &owned;
let borrowed: &str = owned.as_str();

// String → &str slice
let first: &str = &owned[..2]; // "hi"[..2] = "hi"

// String → owned differently
let upper: String = owned.to_uppercase(); // returns new String
```

### Why pass `&str` not `&String` to functions?
```rust
fn print(s: &str) { println!("{s}"); }    // flexible: accepts &str, &String, literals
fn print(s: &String) { println!("{s}"); } // less flexible: only &String
```

Prefer `&str` parameters — Rust auto-coerces `&String` to `&str` via `Deref`.

---

## DSTs (Dynamically Sized Types)

Three DSTs in Rust: `str`, `[T]` (slice), `dyn Trait`.

All DSTs share one property: size is not known at compile time, so they must be behind a pointer.

```rust
// All compile errors:
let s: str = ...;         // can't size on stack
let v: [i32] = ...;       // can't size on stack
let t: dyn Display = ...; // can't size on stack

// Correct:
let s: &str = "hi";
let v: &[i32] = &[1, 2, 3];
let t: &dyn Display = &42;
let t: Box<dyn Display> = Box::new(42);
```

---

## Primitives

```rust
// Integers: signed (i8, i16, i32, i64, i128, isize), unsigned (u8..u128, usize)
let x: i32 = -5;
let y: u64 = 1_000_000; // underscores for readability
let z: usize = 42;      // pointer-sized, used for indexing

// Float: f32, f64 (default is f64)
let f: f64 = 3.14;

// Boolean
let b: bool = true;

// Char: 4 bytes, Unicode scalar value (not a byte!)
let c: char = '✓';

// Unit type (empty tuple): ()
fn do_nothing() -> () {}
```

### Integer overflow
```rust
fn main() {
    let x: u8 = 255;
    let y = x + 1; // ?
}
```
In **debug build**: runtime panic "attempt to add with overflow".  
In **release build**: wraps around (255u8 + 1 = 0).

Use `.wrapping_add()`, `.checked_add()`, or `.saturating_add()` for explicit behavior.

---

## Tuples and Arrays

```rust
// Tuple: fixed size, mixed types, access by index
let t: (i32, &str, bool) = (1, "hi", true);
println!("{} {} {}", t.0, t.1, t.2);
let (a, b, c) = t; // destructure

// Array: fixed size, same type, stack-allocated
let arr: [i32; 3] = [1, 2, 3];
let zeros = [0i32; 100]; // 100 zeros

// Slice: borrowed view into array or Vec
let sl: &[i32] = &arr[..2]; // [1, 2]
let sl: &[i32] = &arr;      // whole array as slice
```

---

## Option<T>

Replaces null. Forces you to handle the "no value" case.

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some("Alice".into()) } else { None }
}

fn main() {
    // match
    match find_user(1) {
        Some(name) => println!("found: {name}"),
        None       => println!("not found"),
    }

    // if let (when you only care about Some)
    if let Some(name) = find_user(1) {
        println!("{name}");
    }

    // unwrap — panics if None (use sparingly)
    let name = find_user(1).unwrap();

    // default fallback
    let name = find_user(99).unwrap_or("unknown".into());
    let name = find_user(99).unwrap_or_default(); // "" for String

    // transform the inner value
    let len: Option<usize> = find_user(1).map(|n| n.len());

    // chain Options
    let upper = find_user(1).and_then(|n| {
        if n.len() > 3 { Some(n.to_uppercase()) } else { None }
    });
}
```

---

## Result<T, E>

For operations that can fail with a specific error type.

```rust
fn parse_age(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}

fn main() {
    match parse_age("25") {
        Ok(age) => println!("age: {age}"),
        Err(e)  => println!("error: {e}"),
    }
}
```

### The `?` Operator

Propagates the error up the call stack. Equivalent to an early `return Err(e)`.

```rust
fn parse_and_double(s: &str) -> Result<u32, std::num::ParseIntError> {
    let n = s.parse::<u32>()?; // returns early with Err if parse fails
    Ok(n * 2)
}
```

Expanded form of what `?` does:
```rust
let n = match s.parse::<u32>() {
    Ok(v)  => v,
    Err(e) => return Err(e.into()),
};
```

### What happens?
```rust
fn main() {
    std::fs::read_to_string("file.txt")?; // ?
}
```
**Compile error**: `?` requires the function to return `Result` or `Option`.

**Fix**: change main's return type
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{content}");
    Ok(())
}
```

---

## Error Handling Libraries

### `thiserror` — for libraries (typed errors callers can match on)
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("user not found: id={id}")]
    NotFound { id: u32 },

    #[error("permission denied")]
    Unauthorized,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error), // auto-converts std::io::Error via ?
}
```

Callers can pattern match:
```rust
match process() {
    Err(AppError::NotFound { id }) => println!("no user {id}"),
    Err(AppError::Unauthorized) => println!("denied"),
    _ => {}
}
```

### `anyhow` — for application binaries (ergonomic propagation)
```rust
use anyhow::{Result, Context};

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config from {path}"))?;
    Ok(parse(content))
}
```

`anyhow::Result<T>` = `Result<T, anyhow::Error>`. Works with any error type automatically via `?`. Use in `main`, CLI tools, and application code where you just need to surface errors, not match on them.

### Decision rule
- **`thiserror`**: library crate, callers need to handle specific error variants.
- **`anyhow`**: binary crate, you just need to propagate and display errors.
