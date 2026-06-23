# Generics and Dispatch

## Mental Model
- Generics (`<T>`) let one function/struct work with many types.
- Trait bounds (`T: Display`) constrain what `T` can be.
- **Static dispatch**: compiler generates specialized code per type — fast, zero overhead.
- **Dynamic dispatch** (`dyn Trait`): type resolved at runtime via vtable — flexible, slight overhead.

---

## Generic Without Trait Bound

### What happens?
```rust
fn print_it<T>(val: T) {
    println!("{}", val);
}
```
**Compile error**: `T` doesn't implement `Display`.

`println!("{}", ...)` requires `std::fmt::Display`. A raw `T` with no bounds is completely unknown.

**Fix**: add a bound
```rust
use std::fmt::Display;

fn print_it<T: Display>(val: T) {
    println!("{}", val);
}
```

---

## Multiple Bounds

```rust
use std::fmt::{Debug, Display};

fn log_value<T: Display + Debug>(val: T) {
    println!("display: {val}");
    println!("debug:   {val:?}");
}
```

`+` chains multiple bounds. `T` must satisfy all of them.

**`where` clause** — same meaning, cleaner for complex signatures:
```rust
fn process<T, U>(a: T, b: U) -> String
where
    T: Display + Clone,
    U: Debug,
{
    format!("{a} {:?}", b)
}
```

---

## Generic Struct

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn show_larger(&self) {
        if self.first > self.second {
            println!("{}", self.first);
        } else {
            println!("{}", self.second);
        }
    }
}
```

The `impl<T: ...>` means: only implement `show_larger` when `T` satisfies those bounds.

---

## Static Dispatch (Monomorphization)

```rust
trait Greet {
    fn hello(&self) -> String;
}

struct English;
struct Spanish;

impl Greet for English { fn hello(&self) -> String { "Hello".into() } }
impl Greet for Spanish { fn hello(&self) -> String { "Hola".into()  } }

fn greet<T: Greet>(g: T) {
    println!("{}", g.hello());
}

fn main() {
    greet(English);
    greet(Spanish);
}
```

At compile time, Rust generates two separate functions:
- `greet_English` → calls `English::hello` directly
- `greet_Spanish` → calls `Spanish::hello` directly

No runtime lookup. Called **monomorphization**. Zero overhead.

**Tradeoff**: binary grows per concrete type. Cannot mix `English` and `Spanish` in the same `Vec<T>`.

---

## Dynamic Dispatch (`dyn Trait`)

```rust
trait Greet {
    fn hello(&self) -> String;
}

fn greet_dyn(g: &dyn Greet) {
    println!("{}", g.hello());
}

fn main() {
    let items: Vec<Box<dyn Greet>> = vec![
        Box::new(English),
        Box::new(Spanish),
    ];
    for item in &items {
        greet_dyn(item.as_ref());
    }
}
```

`dyn Greet` is a fat pointer: `(data_ptr, vtable_ptr)`. The vtable maps method calls to implementations at runtime.

**Use when**: mixing types in a collection, plugin systems, returning different concrete types from a function.

---

## `impl Trait` in Return Position

```rust
fn make_greeter() -> impl Greet {
    English // concrete type hidden from caller, but fixed at compile time
}
```

Static dispatch — compiler knows the concrete type. The caller just can't name it.

### What happens when you return different types?
```rust
fn pick(flag: bool) -> impl Greet {
    if flag { English } else { Spanish } // compile error
}
```
**Compile error**: `if` and `else` have incompatible types.

`impl Trait` means one hidden type — you can't return two different types.

**Fix**: use `Box<dyn Greet>`
```rust
fn pick(flag: bool) -> Box<dyn Greet> {
    if flag { Box::new(English) } else { Box::new(Spanish) }
}
```

---

## `impl Trait` in Parameter Position

```rust
fn notify(item: &impl Summary) {          // syntax sugar
    println!("{}", item.summarize());
}

fn notify<T: Summary>(item: &T) {         // equivalent, more explicit
    println!("{}", item.summarize());
}
```

These are identical. `impl Trait` in parameter position is just shorthand for a generic bound.

---

## Static vs Dynamic: Decision Guide

| | Generic `<T: Trait>` | `dyn Trait` |
|---|---|---|
| Dispatch | compile-time | runtime (vtable) |
| Overhead | zero | one pointer indirection |
| Heterogeneous collection | no | yes |
| Binary size | grows per type | constant |
| Return different types | no | yes |

**Rule of thumb**: prefer generics for performance. Use `dyn Trait` when you need runtime polymorphism or heterogeneous collections.

---

## Demo: Can You Demonstrate Both Dispatches?

```rust
trait Area {
    fn area(&self) -> f64;
}

struct Circle { r: f64 }
struct Square { s: f64 }

impl Area for Circle { fn area(&self) -> f64 { 3.14 * self.r * self.r } }
impl Area for Square { fn area(&self) -> f64 { self.s * self.s } }

// Static dispatch — T resolved at compile time
fn print_area_static<T: Area>(shape: &T) {
    println!("{}", shape.area());
}

// Dynamic dispatch — resolved at runtime
fn print_area_dynamic(shape: &dyn Area) {
    println!("{}", shape.area());
}

fn main() {
    let c = Circle { r: 2.0 };
    let s = Square { s: 3.0 };

    print_area_static(&c);   // monomorphized
    print_area_static(&s);   // monomorphized

    // Can mix types — only possible with dyn
    let shapes: Vec<Box<dyn Area>> = vec![Box::new(c), Box::new(s)];
    for shape in &shapes {
        print_area_dynamic(shape.as_ref());
    }
}
```
