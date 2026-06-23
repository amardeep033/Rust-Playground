# Doubts and Practice Prompts

## Interview-Style Snippets: What Happens?

### Ownership & Borrowing

**1. Will this compile?**
```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{r1} {r2} {r3}");
}
```
<details>
<summary>Answer</summary>
Compile error. `r1` and `r2` are active immutable borrows when `r3` tries to take a mutable borrow. All three are used in `println!`, so the borrows overlap.
</details>

---

**2. What's the difference?**
```rust
let a = &mut b;    // (A)
let mut a = &b;    // (B)
```
<details>
<summary>Answer</summary>

- (A): `a` holds a mutable reference to `b`. `b`'s contents can be mutated through `a`. The binding `a` itself is not reassignable.
- (B): `a` holds an immutable reference to `b`. `b` cannot be mutated through `a`. But the *binding* `a` can be reassigned to point elsewhere.

```rust
let mut b = 5;
let a = &mut b;  // can do: *a = 10 (mutates b)
                 // cannot: a = &mut other_var (binding immutable)

let mut a = &b;  // cannot: *a = 10 (immutable ref)
                 // can do: a = &other_var (binding is mut)
```
</details>

---

**3. NLL question — will this compile?**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}
```
<details>
<summary>Answer</summary>
Compile error. `first` borrows from `v`. `v.push(4)` requires `&mut v`. The immutable borrow `first` is still alive because it's used in `println!` after the push. If you move `println!` before the push, it compiles.
</details>

---

**4. What's the output?**
```rust
fn main() {
    let x = 5;
    let y = x;
    println!("{x} {y}");
}
```
<details>
<summary>Answer</summary>
Prints `5 5`. `i32` is `Copy`. Both `x` and `y` are valid independent copies.
</details>

---

### Lifetimes

**5. Fix this function:**
```rust
fn shorter(x: &str, y: &str) -> &str {
    if x.len() < y.len() { x } else { y }
}
```
<details>
<summary>Answer</summary>

```rust
fn shorter<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() { x } else { y }
}
```
The output reference might be either `x` or `y`, so the compiler needs to know the output is bounded by whichever is shorter-lived.
</details>

---

**6. Does this compile? Why or why not?**
```rust
struct S<'a> { val: &'a str }

fn main() {
    let s: S;
    {
        let text = String::from("hi");
        s = S { val: &text };
    }
    println!("{}", s.val);
}
```
<details>
<summary>Answer</summary>
Compile error. `text` is dropped at the end of the inner block. `s` holds a reference to `text`, and `s` is used after `text` is gone. The lifetime annotation `'a` enforces this constraint — the struct can't outlive the data it references.
</details>

---

**7. When does lifetime elision apply? Name the three rules.**
<details>
<summary>Answer</summary>

1. Each reference parameter gets its own independent lifetime.
2. If there's exactly one input lifetime, it's used for all output lifetimes.
3. If one parameter is `&self` or `&mut self`, its lifetime is used for all outputs.

```rust
fn foo(x: &str) -> &str       // rule 2: output = same as x's lifetime
impl S { fn get(&self) -> &str }  // rule 3: output = self's lifetime
```
</details>

---

### Generics & Dispatch

**8. Can you store `Dog` and `Cat` in the same Vec if both implement `Animal`? How?**
<details>
<summary>Answer</summary>

```rust
// Not with generics:
let v: Vec<impl Animal> = ...; // each element must be the same type

// Yes with trait objects:
let v: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
```
`dyn Trait` allows heterogeneous collections. `Box` gives each element the same known size (a pointer).
</details>

---

**9. What's wrong with this?**
```rust
fn make_animal(kind: &str) -> impl Animal {
    if kind == "dog" { Dog } else { Cat }
}
```
<details>
<summary>Answer</summary>
Compile error. `impl Trait` in return position means one hidden but concrete type. The `if` branches return different types (`Dog` and `Cat`), which violates this.

Fix:
```rust
fn make_animal(kind: &str) -> Box<dyn Animal> {
    if kind == "dog" { Box::new(Dog) } else { Box::new(Cat) }
}
```
</details>

---

### Smart Pointers

**10. Why does `Rc` fail in a thread?**
<details>
<summary>Answer</summary>
`Rc<T>` uses non-atomic reference counting. If two threads simultaneously drop their clones, both would decrement the counter — classic race condition. Rust prevents this by making `Rc` not `Send`. Use `Arc<T>` which uses atomic operations.
</details>

---

**11. What panics at runtime (not compile time) with `RefCell`?**
<details>
<summary>Answer</summary>
Taking an immutable and mutable borrow simultaneously, or two mutable borrows:
```rust
let x = RefCell::new(5);
let r1 = x.borrow();
let r2 = x.borrow_mut(); // panics: already immutably borrowed
```
`RefCell` moves the borrow check from compile time to runtime. It compiles but panics.
</details>

---

**12. Design a shared counter that 5 threads increment. What type?**
<details>
<summary>Answer</summary>

```rust
let counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
```
- `Arc<T>`: shared ownership across threads.
- `Mutex<T>`: ensures only one thread mutates at a time.
</details>

---

### Concurrency & Async

**13. Why is `std::thread::sleep` bad inside async?**
<details>
<summary>Answer</summary>
It blocks the OS thread that the async executor is running on. Tokio's executor shares a thread pool. If one thread is blocked, every other task on that thread is frozen too. Use `tokio::time::sleep(duration).await` instead — it suspends the task without blocking the thread.
</details>

---

**14. `join!` vs `select!` — when do you use which?**
<details>
<summary>Answer</summary>

- `join!(a, b)`: wait for **all** futures to complete. Good when you need results from all.
- `select! { r = a() => ..., r = b() => ... }`: wait for the **first** future to complete, cancel the rest. Good for timeouts, fallback fetches, racing connections.
</details>

---

**15. What constraint must a future satisfy for `tokio::spawn`?**
<details>
<summary>Answer</summary>
`Future + Send + 'static`. The `Send` bound is needed because spawned tasks can run on any thread in the pool. The `'static` bound means the future can't borrow from the current stack frame (the task may outlive the spawner).
</details>

---

### Error Handling

**16. When do you choose `thiserror` over `anyhow`?**
<details>
<summary>Answer</summary>

- **`thiserror`**: writing a library. Callers need to match on specific error variants. Gives you typed, nameable error enums.
- **`anyhow`**: writing an application binary. You just need to propagate and display errors. `?` works with any error type automatically.
</details>

---

**17. What does `?` do exactly? What must the function return?**
<details>
<summary>Answer</summary>

`?` is sugar for:
```rust
match expr {
    Ok(v)  => v,
    Err(e) => return Err(e.into()),
}
```
The function must return `Result<_, E>` (or `Option<_>`). The `.into()` means the error type must be convertible to the function's error type via `From`.
</details>

---

## Open Questions to Investigate

- [ ] How does `Pin<T>` work with async? Why do some futures need to be pinned?
- [ ] What is `Unpin` and when do you implement it?
- [ ] How does `Deref` coercion work? Why can you pass `&String` where `&str` is expected?
- [ ] What is a self-referential struct and why does async code sometimes create one?
- [ ] `impl Trait` vs `dyn Trait` in function parameters — any performance difference?
- [ ] How does the borrow checker handle iterator methods like `.map()`? Why does `v.iter().map(|x| v.len())` sometimes fail?
- [ ] What is `CoerceUnsized` and how do DST coercions work internally?
- [ ] What's the difference between `tokio::task::JoinHandle` and `std::thread::JoinHandle`?

---

## Quick Self-Test Checklist

When reading any Rust snippet:
1. Non-Copy type assigned/passed twice without `.clone()`? → move error
2. Borrow still active when move or second borrow happens? → borrow conflict
3. `&T` and `&mut T` overlap in time? → aliasing error
4. Returned reference ties to a local variable? → dangling ref
5. `T` used with `{}` or `.clone()` without a bound? → trait bound error
6. `Rc` used across thread boundary? → not Send
7. `dyn Trait` without `&` or `Box`? → DST error
8. `RefCell` borrow overlaps → compiles, panics at runtime
9. `?` used in function not returning `Result`/`Option`? → type error
10. Blocking call inside async fn? → compiles, executor starvation at runtime
