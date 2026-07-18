# Static vs Dynamic Dispatch

## Problem

You have a common `Animal` trait implemented by multiple concrete types (`Dog`, `Cat`). Write a function that can speak on behalf of any `Animal` without knowing the concrete type at compile time.

Compare two approaches:

1. **Static dispatch** — a generic function `fn make_speak<T: Animal>(animal: &T)`, monomorphized per type at compile time.
2. **Dynamic dispatch** — a function taking `&dyn Animal`, resolved via vtable at runtime.

## Function Signature

```rust
trait Animal {
    fn speak(&self);
}

fn make_speak(animal: &dyn Animal);
```

## Constraints

- `Dog` and `Cat` must both implement `Animal`.
- Demonstrate calling `make_speak` with both types through the same function.
- Note the tradeoff: static dispatch is zero-cost but generates more binary code (one copy per type); dynamic dispatch has a small vtable-lookup cost but keeps a single function body and allows heterogeneous collections (e.g. `Vec<Box<dyn Animal>>`).

## Sample Input

```rust
let dog = Dog { name: "Buddy".to_string() };
let cat = Cat { name: "Kitty".to_string() };
```

## Expected Output

```
Buddy says Woof!
Kitty says Meow!
```
