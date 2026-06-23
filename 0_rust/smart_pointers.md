# Smart Pointers

## Overview

| Pointer | Ownership | Thread-safe | Mutability |
|---------|-----------|-------------|------------|
| `Box<T>` | single owner | if T: Send | normal borrow rules |
| `Rc<T>` | shared (ref count) | no | read-only by default |
| `Arc<T>` | shared (atomic ref count) | yes | read-only by default |
| `RefCell<T>` | single owner | no | interior mutability (runtime check) |
| `Rc<RefCell<T>>` | shared | no | shared + mutable, single thread |
| `Arc<Mutex<T>>` | shared | yes | exclusive lock |
| `Arc<RwLock<T>>` | shared | yes | many readers OR one writer |

---

## `Box<T>`: Heap Allocation

Stack stores pointer. Heap stores actual value. Single owner.

### When to use
- Large data you want on heap, not stack.
- Recursive types (compiler needs known size, pointer breaks the cycle).
- Trait objects (`Box<dyn Trait>`).

```rust
fn main() {
    let b = Box::new([0u8; 1024 * 1024]); // 1 MB on heap, not stack
    println!("{}", b[0]);
} // b dropped here, heap memory freed
```

### Recursive type without Box — what happens?
```rust
enum List {
    Cons(i32, List), // compile error
    Nil,
}
```
**Compile error**: recursive type has infinite size.

**Why**: Rust needs to know the size of `List` at compile time. `Cons` contains a `List` which contains a `Cons` which... infinite size.

**Fix**: break cycle with a pointer
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```
`Box<List>` is just a pointer — known size (8 bytes on 64-bit). The heap holds the actual `List`.

---

## `Box<dyn Trait>`: Trait Objects

### Why not `Vec<dyn Animal>`?
```rust
trait Animal { fn speak(&self); }
struct Dog;
struct Cat;

impl Animal for Dog { fn speak(&self) { println!("woof"); } }
impl Animal for Cat { fn speak(&self) { println!("meow"); } }

fn main() {
    let v: Vec<dyn Animal> = vec![Dog, Cat]; // compile error
}
```
**Compile error**: `dyn Animal` has unknown size at compile time.

`dyn Trait` is a DST (dynamically sized type). Vectors need elements with fixed, known size.

**Fix**: use `Box<dyn Animal>` — the pointer has known size
```rust
fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &animals {
        a.speak();
    }
}
```

### `&dyn Trait` vs `Box<dyn Trait>`
- `&dyn Trait`: borrowed reference to some trait object. No heap allocation by itself.
- `Box<dyn Trait>`: owned, heap-allocated. The box owns the data.

```rust
fn describe(a: &dyn Animal) { a.speak(); } // just borrows

fn make_animal() -> Box<dyn Animal> {       // returns owned, heap data
    Box::new(Dog)
}
```

---

## `Rc<T>`: Shared Ownership (Single Thread Only)

Multiple owners via reference counting. When the last owner drops, the data drops.

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a); // clone the pointer, not the data
    let c = Rc::clone(&a);

    println!("count: {}", Rc::strong_count(&a)); // 3
    println!("{a} {b} {c}");
    drop(b);
    println!("count: {}", Rc::strong_count(&a)); // 2
} // a, c drop → count hits 0 → String freed
```

### What happens when you send Rc to a thread?
```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(42);
    thread::spawn(move || println!("{data}")); // compile error
}
```
**Compile error**: `Rc<i32>` cannot be sent between threads safely — `Rc` is not `Send`.

**Why**: `Rc` uses non-atomic reference counting. Two threads decrementing the count simultaneously = data race.

---

## `Arc<T>`: Shared Ownership (Multi-Thread)

Same as `Rc` but uses atomic operations on the reference count.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(String::from("shared"));
    let data2 = Arc::clone(&data);

    let h = thread::spawn(move || {
        println!("thread: {data2}");
    });

    println!("main: {data}");
    h.join().unwrap();
}
```

---

## `RefCell<T>`: Interior Mutability (Single Thread)

Lets you mutate data through a shared `&T` reference. Borrow rules enforced at **runtime**, not compile time.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);
    data.borrow_mut().push(4); // runtime borrow
    println!("{:?}", data.borrow());
}
```

### Runtime panic — NOT a compile error

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    let r1 = data.borrow();     // immutable borrow
    let r2 = data.borrow_mut(); // panics: already borrowed immutably
}
```
**Compiles fine. Panics at runtime.**

This is the key tradeoff: `RefCell` moves the borrow check from compile time to runtime. You get flexibility but lose the compile-time safety guarantee.

---

## `Rc<RefCell<T>>`: Shared + Mutable (Single Thread)

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;

    println!("{}", shared.borrow()); // 15
}
```

`Rc` gives shared ownership. `RefCell` gives the ability to mutate. Together: shared mutable data in a single thread.

---

## `Arc<Mutex<T>>`: Shared + Mutable (Multi-Thread)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = c.lock().unwrap(); // blocks until lock available
            *guard += 1;
        })); // guard dropped here → lock released
    }

    for h in handles { h.join().unwrap(); }
    println!("{}", counter.lock().unwrap()); // 5
}
```

`lock()` returns a `MutexGuard` that auto-releases when it drops.

### Deadlock — compiles, hangs at runtime
```rust
let m = Mutex::new(0);
let g1 = m.lock().unwrap();
let g2 = m.lock().unwrap(); // hangs forever — g1 never released
```

---

## `Arc<RwLock<T>>`: Read-Heavy Sharing

Multiple readers can hold read locks simultaneously. Writers get exclusive access.

```rust
use std::sync::{Arc, RwLock};

let data = Arc::new(RwLock::new(vec![1, 2, 3]));

// Many simultaneous readers
let r1 = data.read().unwrap();
let r2 = data.read().unwrap();
println!("{r1:?} {r2:?}");
drop(r1); drop(r2);

// One exclusive writer
let mut w = data.write().unwrap();
w.push(4);
```

Use `RwLock` when reads are frequent and writes are rare (e.g., configuration, caches).

---

## Send and Sync

These are marker traits the compiler uses to check thread safety.

- **`Send`**: it's safe to transfer *ownership* of `T` to another thread.
- **`Sync`**: it's safe to share `&T` across threads (`&T: Send` iff `T: Sync`).

| Type | Send | Sync | Why |
|------|------|------|-----|
| `i32`, `String` | yes | yes | plain data |
| `Rc<T>` | no | no | non-atomic refcount |
| `Arc<T>` | yes* | yes* | atomic refcount |
| `RefCell<T>` | yes* | no | runtime borrow not thread-safe |
| `Mutex<T>` | yes* | yes* | provides synchronization |
| `*mut T` (raw pointer) | no | no | unsafe, opt-in |

*when T: Send / T: Sync respectively

### What happens?
```rust
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    let data = Arc::new(RefCell::new(0));
    std::thread::spawn(move || {
        *data.borrow_mut() += 1; // compile error
    });
}
```
**Compile error**: `RefCell<i32>` cannot be shared between threads safely.

`Arc<RefCell<T>>` won't compile because `RefCell` is not `Sync`. Use `Arc<Mutex<T>>` instead.
