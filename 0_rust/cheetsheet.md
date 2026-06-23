# Rust Interview Cheatsheet

## Ownership Rules
- Every value has exactly one owner.
- Owner goes out of scope → value dropped.
- Non-Copy assignment = move, not copy.
- Borrow ends at **last use** (NLL), not at `}`.

## Borrow Rules
- Many `&T` OR exactly one `&mut T` — never both.
- No borrow can outlive the owner.

## Copy vs Move
```
Copy  (duplicated):  i32, u64, f64, bool, char, &T, tuples/arrays of Copy
Move  (transferred): String, Vec<T>, Box<T>, any heap-owning type
```

## String Types
```
str      — DST, can't use directly, always behind & or Box
&str     — fat pointer (ptr + len), borrows UTF-8 data, immutable
String   — (ptr + len + cap), heap-owned, mutable
```
```rust
let a: &str    = "literal";              // &'static str
let b: String  = "hi".to_string();       // owned
let c: &str    = &b;                     // borrow from String
let d: &str    = &b[1..];               // slice
```

## Lifetimes
```rust
fn f<'a>(x: &'a str, y: &'a str) -> &'a str { ... }  // output tied to inputs
struct S<'a> { val: &'a str }                          // struct holds ref
impl<'a> S<'a> { fn get(&self) -> &str { self.val } }  // elision in impl
```
Elision rules: 1 input ref → output gets same lifetime. `&self` method → output tied to self.

## Dispatch
```rust
fn f<T: Trait>(x: T) {}       // static — monomorphized, zero overhead
fn f(x: &dyn Trait) {}        // dynamic — vtable, flexible
fn f() -> impl Trait {}       // static, hidden concrete type
fn f() -> Box<dyn Trait> {}   // dynamic, heap, can return different types
```

## Smart Pointers
```rust
Box::new(v)                   // heap, single owner, DST/recursive types
Rc::new(v) / Rc::clone(&r)    // shared ownership, single thread
Arc::new(v) / Arc::clone(&a)  // shared ownership, multi-thread
RefCell::new(v)               // interior mutability (runtime checks, single thread)
```

## Shared Mutable Patterns
```rust
Rc<RefCell<T>>    // single thread: shared + mutable
Arc<Mutex<T>>     // multi-thread: exclusive lock
Arc<RwLock<T>>    // multi-thread: many readers OR one writer
```

## Send + Sync
```
Send   — safe to move ownership to another thread
Sync   — safe to share &T across threads

Rc<T>        → not Send, not Sync
Arc<T>       → Send + Sync (if T: Send+Sync)
RefCell<T>   → Send (if T: Send), NOT Sync
Mutex<T>     → Send + Sync (if T: Send)
```

## Option<T>
```rust
opt.unwrap()              // panic if None
opt.expect("msg")         // panic with message if None
opt.unwrap_or(default)    // value or default
opt.unwrap_or_default()   // value or Default::default()
opt.map(|v| ...)          // transform Some, pass None through
opt.and_then(|v| ...)     // chain: f returns Option
opt.is_some() / is_none()
```

## Result<T, E>
```rust
res.unwrap()              // panic if Err
res.expect("msg")         // panic with message if Err
res.unwrap_or(default)
res.map(|v| ...)          // transform Ok
res.map_err(|e| ...)      // transform Err
res?                      // propagate Err (fn must return Result/Option)
res.ok()                  // Result → Option (drops error)
```

## Error Libraries
```
thiserror  — typed error enums for libraries (callers match on variants)
anyhow     — ergonomic propagation for binaries (just surface errors)
```

## Async
```rust
async fn foo() -> T { ... }          // returns Future<Output = T>
foo().await                           // drive future to completion
tokio::spawn(async { ... })           // spawn task (must be Send)
join!(a(), b())                       // wait all concurrently
select! { r = a() => ..., r = b() => ... }  // wait first
spawn_blocking(|| heavy_work())      // offload blocking/CPU work
```

## Traits Quick Reference
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
trait Foo { fn bar(&self) -> String; fn baz(&self) -> u32 { 0 } }  // default impl
impl<T: Display> Printable for T { ... }  // blanket impl
```

## Common Trait Bounds
```rust
T: Display           // println!("{}", v)
T: Debug             // println!("{:?}", v)
T: Clone             // v.clone()
T: Copy              // implicitly copied on assign
T: Send + Sync       // safe across threads
T: 'static           // no non-static borrows
T: Default           // T::default()
```

## Iterator Methods
```rust
.map(|x| x * 2)           .filter(|x| *x > 0)
.fold(0, |acc, x| acc+x)  .reduce(|a, b| a+b)
.collect::<Vec<_>>()       .count()
.enumerate()               .zip(other)
.flat_map(|x| x.iter())    .chain(other)
.any(|x| x > 0)            .all(|x| x > 0)
.find(|x| *x == v)         .position(|x| *x == v)
.take(n)                   .skip(n)
.sum::<i32>()              .min() / .max()
.cloned()                  .copied()
```

## Cargo Commands
```bash
cargo check             # type check, no binary
cargo build             # debug build
cargo build --release   # optimized build
cargo run               # build + run
cargo test              # run tests
cargo clippy            # lints
cargo fmt               # format
cargo doc --open        # generate + open docs
cargo add <crate>       # add dependency
```

## Quick Compile Error Checklist
1. Non-Copy used after assignment/function call → move error
2. Borrow active when move or second borrow happens → borrow conflict
3. `&T` and `&mut T` overlap in time → borrow conflict
4. Returned reference to local variable → dangling ref
5. `T` used with `{}` / `.clone()` without bound → trait bound error
6. `Rc` used across thread boundary → not Send error
7. `dyn Trait` not behind `&` or `Box` → DST error
8. `RefCell` double-borrow → runtime panic (not compile error!)
9. `?` in `main()` without `Result` return type → type mismatch
10. Blocking I/O inside async fn → no compile error, but starves executor
