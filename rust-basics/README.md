# rust-basics

Runnable ownership, borrowing, lifetime, `Rc`, `Box`, and string-slice examples.

## What it shows

- move semantics and why `let b = a;` moves a `String`
- why moving while borrowed fails if the borrow is still used later
- why immutable and mutable borrows cannot overlap
- when explicit lifetimes are needed for returned references
- shared ownership with `Rc<T>`
- why `as_ref()` lets you inspect an owned value without moving it
- what `Box<T>` stores on the heap
- why `str` is unsized and why owned `String` values fix mutation/lifetime issues

## Run

```bash
cargo run
```

Each section prints a small example so you can compare the working pattern with the compiler errors from the original snippets.