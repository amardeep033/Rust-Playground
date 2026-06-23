# Rust Compile Errors (and Runtime Traps)

Format: snippet → what happens → why → fix.

---

## E0382 — Use of Moved Value

```rust
let a = String::from("hi");
let b = a;
let c = a; // error[E0382]
```
**Why**: `String` is non-Copy. `let b = a` transfers ownership to `b`. `a` is now invalid (moved).  
**Fix**: clone before moving, or restructure to use `a` only once.
```rust
let b = a.clone();
let c = a;
```

---

## E0505 — Cannot Move: Still Borrowed

```rust
let a = String::from("hello");
let r = &a;
let b = a;        // error[E0505]
println!("{r}");
```
**Why**: `r` borrows `a`. Moving `a` would leave `r` pointing at freed memory (dangling pointer).  
**Fix**: use the borrow before the move.
```rust
let r = &a;
println!("{r}"); // borrow ends here
let b = a;       // move now ok
```

---

## E0502 — Cannot Borrow as Mutable: Already Immutably Borrowed

```rust
let mut a = String::from("hi");
let r1 = &a;
let r2 = &mut a; // error[E0502]
println!("{r1} {r2}");
```
**Why**: `r1` is an active `&a` borrow. Taking `&mut a` simultaneously breaks the aliasing rule.  
**Fix**: use `r1` completely, then take the mutable borrow.
```rust
println!("{r1}"); // r1 ends here
let r2 = &mut a;
```

---

## E0499 — Cannot Borrow as Mutable More Than Once

```rust
let mut a = String::from("hi");
let r1 = &mut a;
let r2 = &mut a; // error[E0499]
println!("{r1} {r2}");
```
**Why**: two simultaneous mutable borrows. If both could mutate `a`, that's a data race.  
**Fix**: use `r1` completely, then take `r2`.

---

## E0106 — Missing Lifetime Specifier

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```
**Why**: return could be `x` or `y`. Compiler can't know if the returned reference will outlive either.  
**Fix**: tie output lifetime to inputs.
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## E0515 — Cannot Return Reference to Local Variable

```rust
fn make() -> &String {
    let s = String::from("hello");
    &s // s dropped at end of fn
}
```
**Why**: `s` is dropped when the function returns. The returned `&s` would be a dangling pointer.  
**Fix**: return owned data.
```rust
fn make() -> String {
    String::from("hello")
}
```

---

## E0277 — Trait Bound Not Satisfied

```rust
fn print_it<T>(val: T) {
    println!("{}", val); // error: T doesn't impl Display
}
```
**Why**: `{}` in `println!` requires `std::fmt::Display`. `T` has no bounds.  
**Fix**: add the bound.
```rust
fn print_it<T: std::fmt::Display>(val: T) {
    println!("{}", val);
}
```

---

## E0277 — Type Not `Send` (Thread Spawn)

```rust
use std::rc::Rc;
let data = Rc::new(5);
std::thread::spawn(move || println!("{data}")); // error
```
**Why**: `Rc<T>` is not `Send` — its reference count is non-atomic. Two threads modifying it simultaneously = data race.  
**Fix**: use `Arc<T>`.
```rust
use std::sync::Arc;
let data = Arc::new(5);
std::thread::spawn(move || println!("{data}"));
```

---

## E0072 — Recursive Type Has Infinite Size

```rust
enum Tree {
    Node(i32, Tree), // error[E0072]
    Leaf,
}
```
**Why**: `Tree` contains `Tree` — infinite size, can't be computed at compile time.  
**Fix**: use `Box` to break the cycle with an indirection.
```rust
enum Tree {
    Node(i32, Box<Tree>),
    Leaf,
}
```

---

## E0596 — Cannot Borrow as Mutable (Immutable Binding)

```rust
let s = String::from("hi");
s.push_str("!"); // error[E0596]
```
**Why**: `s` is not declared `mut`. Cannot call `&mut self` methods on it.  
**Fix**: `let mut s = ...`

---

## E0308 — `?` Used in `main()` Without `Result` Return

```rust
fn main() {
    std::fs::read_to_string("file.txt")?; // error[E0308]
}
```
**Why**: `?` propagates `Err` up by doing an early return. `main()` returns `()` by default, not `Result`.  
**Fix**:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{content}");
    Ok(())
}
```

---

## DST Not Behind Pointer

```rust
let x: str = *"hello";    // error
let y: dyn Display = 42;  // error
```
**Why**: `str`, `[T]`, and `dyn Trait` are dynamically sized types. Size unknown at compile time — can't exist on the stack directly.  
**Fix**: always put them behind `&`, `Box`, `Rc`, etc.
```rust
let x: &str = "hello";
let y: &dyn Display = &42;
let y: Box<dyn Display> = Box::new(42);
```

---

## Object Safety Violation

```rust
trait MyClone {
    fn clone_it(&self) -> Self;
}
fn use_it(_x: &dyn MyClone) {} // error
```
**Why**: `dyn Trait` uses a vtable where each entry has a fixed layout. `-> Self` returns a different type for every impl — can't be stored in the vtable.  
**Fix**: either make the method generic, or avoid `dyn` and use generics instead.

---

## Closure Borrows After Move

```rust
let name = String::from("Alice");
let greet = || println!("{name}");  // borrows name
let other = name;                   // move — error
greet();
```
**Why**: `greet` borrows `name`. Moving `name` invalidates the borrow.  
**Fix**: move into the closure, or use `name` after the closure is done.
```rust
let greet = move || println!("{name}"); // name moved into closure
greet();
```

---

## Runtime Traps (Compile Fine, Fail at Runtime)

### RefCell double-borrow → panic
```rust
use std::cell::RefCell;
let x = RefCell::new(5);
let r1 = x.borrow();
let r2 = x.borrow_mut(); // compiles! panics at runtime
```
`RefCell` moves borrow checking to runtime. Violating it panics, not compile error.

### Integer overflow → panic in debug, wraps in release
```rust
let x: u8 = 255;
let y = x + 1; // debug: panics; release: y = 0
```

### Mutex deadlock → hangs forever
```rust
let m = Mutex::new(0);
let g = m.lock().unwrap();
let g2 = m.lock().unwrap(); // deadlocks — g never released
```

### Unwrap on None/Err → panic
```rust
let v: Option<i32> = None;
let x = v.unwrap(); // panics: called `Option::unwrap()` on a `None` value
```

### Stack overflow → process crash
```rust
fn foo() { foo(); } // infinite recursion, no compile error
```
