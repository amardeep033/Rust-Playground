# Ownership and Borrowing

## Mental Model
Three rules Rust enforces at compile time:
1. Every value has exactly one owner.
2. When owner goes out of scope → value is dropped (memory freed).
3. Assignment of non-Copy type = ownership **moves**, not copies.

Plus borrowing rules:
- `&T`: immutable borrow — many at once, no mutation.
- `&mut T`: mutable borrow — exactly one, no other borrows simultaneously.
- Borrow ends at **last use** (NLL), not at `}`.

---

## Move vs Copy

### What happens?
```rust
fn main() {
    let a = String::from("hello");
    let b = a;
    println!("{a}"); // ?
}
```
**Compile error**: use of moved value `a`.

`String` is heap-allocated and non-Copy. `let b = a` transfers ownership to `b`. `a` is invalid.

```rust
fn main() {
    let x: i32 = 5;
    let y = x;
    println!("{x} {y}"); // works fine
}
```
`i32` is `Copy`. Stack-only types that are cheap to duplicate implement `Copy`. Assignment copies bits, doesn't move.

**Which types are Copy?**
- Scalar: `i32`, `u64`, `f64`, `bool`, `char`
- References: `&T` (pointer is Copy, not the underlying data)
- Tuples/arrays where all elements are Copy: `(i32, bool)`, `[i32; 3]`

**Which types are NOT Copy (they move)?**
- `String`, `Vec<T>`, `Box<T>` — own heap memory
- Any struct/enum containing a non-Copy field

---

## Moving Into Functions

### What happens?
```rust
fn take(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hi");
    take(s);
    println!("{s}"); // ?
}
```
**Compile error**: value used after move. `take(s)` moves `s` into the function. After the call, `s` is gone.

**Fix**: pass by reference
```rust
fn take(s: &str) {
    println!("{s}");
}

fn main() {
    let s = String::from("hi");
    take(&s);
    println!("{s}"); // fine — s still owns data
}
```

---

## Cannot Move While Borrowed

### What happens?
```rust
fn main() {
    let a = String::from("hello");
    let r = &a;
    let b = a;        // move while r borrows a
    println!("{r}");
}
```
**Compile error**: cannot move out of `a` because it is borrowed.

`r` still references `a`. Moving `a` would leave `r` pointing at freed memory (dangling pointer).

**Fix**: use the borrow before moving
```rust
fn main() {
    let a = String::from("hello");
    let r = &a;
    println!("{r}"); // last use of r — borrow ends here
    let b = a;       // move ok now
    println!("{b}");
}
```

---

## Mutable + Immutable Borrow Conflict

### What happens?
```rust
fn main() {
    let mut a = String::from("hi");
    let r1 = &a;
    let r2 = &mut a;
    println!("{r1} {r2}");
}
```
**Compile error**: cannot borrow `a` as mutable because it is also borrowed as immutable.

**The rule**: At any point you can have *either*:
- Any number of `&T` (read-only borrows), OR
- Exactly one `&mut T` (exclusive write borrow)

Never both simultaneously.

**Fix**: end immutable borrow before taking mutable
```rust
fn main() {
    let mut a = String::from("hi");
    let r1 = &a;
    println!("{r1}"); // r1's last use — borrow ends here
    let r2 = &mut a;
    r2.push_str("!");
    println!("{r2}");
}
```

---

## Two Mutable Borrows at Once

### What happens?
```rust
fn main() {
    let mut s = String::from("hello");
    let a = &mut s;
    let b = &mut s;
    println!("{a} {b}");
}
```
**Compile error**: cannot borrow `s` as mutable more than once at a time.

**Why**: if two `&mut` references existed simultaneously, both could mutate the same data — data race territory.

---

## NLL — Borrow Ends at Last Use (Not at `}`)

### Important interview trap
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}"); // r1, r2 both end here — not at }

    let r3 = &mut s;       // valid: r1, r2 already done
    r3.push_str("!");
    println!("{r3}");
}
```
**This compiles.** NLL (Non-Lexical Lifetimes): borrow is only "active" while it's used.

### Without NLL (old Rust would reject this):
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);         // compile error: can't mutate while first borrows
    println!("{first}");
}
```
**Compile error**: cannot borrow `v` as mutable because it is also borrowed as immutable.

Even though `first` is just an integer reference, the compiler sees the borrow to `v` still active through the `println!` at the end.

---

## Returning References: the Dangling Reference Trap

### What happens?
```rust
fn bad() -> &String {
    let s = String::from("hello");
    &s // s drops here
}
```
**Compile error**: returns a reference to data owned by the current function.

`s` is dropped when `bad()` returns. Returning `&s` would be a dangling pointer.

**Fix**: return owned data
```rust
fn good() -> String {
    String::from("hello") // ownership moves to caller
}
```

---

## Demo: Can You Mutate Through an Immutable Reference?

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    r.push_str("!"); // ?
}
```
**Compile error**: cannot borrow `*r` as mutable, as it is behind a `&` reference.

The `&` borrow is read-only. The binding `s` itself would also need to be `mut`.

---

## Key Decision Table

| Type | Behavior on Assignment | Reason |
|------|----------------------|--------|
| `i32`, `bool`, `char`, `f64` | Copy | Stack-only, trivially cheap to duplicate |
| `String`, `Vec<T>`, `Box<T>` | Move | Own heap memory — one owner enforced |
| `&T` reference | Copy | Pointer itself is stack-only; borrows data |
| `&mut T` | Move | Exclusive access must not be duplicated |

---

## Quick Checklist for Any Snippet

1. Is a non-Copy value used after assignment/pass to function? → move error
2. Is there a borrow still active when you try to move? → borrow error
3. Do immutable and mutable borrows overlap? → conflict error
4. Is a mutable borrow taken twice? → exclusive borrow error
5. Does a return reference tie to a local variable? → dangling ref error
