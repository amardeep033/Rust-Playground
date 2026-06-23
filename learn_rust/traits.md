# Traits

## Mental Model
A trait is a contract: "any type implementing this trait guarantees these methods exist."
Rust has **no inheritance**. Shared behavior comes entirely from traits.
Composition > Inheritance is the Rust philosophy.

---

## Defining and Implementing a Trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    body: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.body[..50.min(self.body.len())])
    }
}

fn main() {
    let a = Article {
        title: "Rust is fast".into(),
        body: "Rust achieves memory safety without GC...".into(),
    };
    println!("{}", a.summarize());
}
```

---

## Default Method Implementation

```rust
trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(by {}...)", self.author()) // default — can be overridden
    }
}

struct Tweet { username: String, content: String }

impl Summary for Tweet {
    fn author(&self) -> String { self.username.clone() }
    // summarize uses the default
}
```

---

## Trait as Function Parameter

```rust
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Equivalent longer form (generic bound):
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

Both mean the same thing. The generic form is needed when you want to reference `T` elsewhere (e.g., return it, or have two params of same type).

---

## Trait Objects: Runtime Polymorphism

### What happens?
```rust
trait Shape { fn area(&self) -> f64; }

fn main() {
    let shapes: Vec<dyn Shape> = vec![]; // ?
}
```
**Compile error**: `dyn Shape` has unknown size at compile time.

**Fix**: use `Box<dyn Shape>`
```rust
struct Circle { r: f64 }
struct Square { side: f64 }

impl Shape for Circle { fn area(&self) -> f64 { 3.14 * self.r * self.r } }
impl Shape for Square { fn area(&self) -> f64 { self.side * self.side } }

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { r: 2.0 }),
        Box::new(Square { side: 3.0 }),
    ];
    println!("{:.2}", total_area(&shapes)); // 22.56
}
```

---

## Object Safety

Not every trait can be used as `dyn Trait`. A trait is **object-safe** if:
- Methods don't return `Self`.
- Methods don't have generic type parameters.

### What happens?
```rust
trait Cloneable {
    fn clone_me(&self) -> Self; // returns Self
}

fn use_it(_x: &dyn Cloneable) {} // compile error
```
**Compile error**: `Cloneable` is not object-safe because method `clone_me` references `Self`.

**Why**: a vtable entry needs a fixed return type. `Self` is different for every concrete type — can't be stored in a vtable.

**This is why `Clone` can't be made into `dyn Clone`.**

---

## Common Standard Traits

```rust
// Debug: {:?} formatting
#[derive(Debug)]
struct Point { x: f64, y: f64 }
let p = Point { x: 1.0, y: 2.0 };
println!("{p:?}"); // Point { x: 1.0, y: 2.0 }

// Display: {} formatting — must implement manually
use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Clone: explicit deep copy
#[derive(Clone)]
struct Config { name: String }
let c1 = Config { name: "prod".into() };
let c2 = c1.clone(); // deep copy

// PartialEq, Eq, PartialOrd, Ord: comparisons
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Score(u32);

// From/Into: type conversions (implement From, get Into free)
struct Meters(f64);
impl From<f64> for Meters {
    fn from(v: f64) -> Self { Meters(v) }
}
let m: Meters = Meters::from(5.0);
let m: Meters = 5.0f64.into(); // Into auto-derived from From
```

---

## OOP in Rust: No Inheritance, Use Composition + Traits

### Encapsulation (fields private by default)
```rust
pub struct BankAccount {
    balance: f64, // private — only accessible through methods
}

impl BankAccount {
    pub fn new(initial: f64) -> Self {
        Self { balance: initial }
    }
    pub fn deposit(&mut self, amount: f64) {
        assert!(amount > 0.0);
        self.balance += amount;
    }
    pub fn balance(&self) -> f64 {
        self.balance
    }
}
```

### Composition Instead of Inheritance
```rust
// Java: class Car extends Vehicle { ... }
// Rust: separate concerns, compose via traits

trait HasEngine { fn start(&self) -> String; }
trait HasSeats  { fn seat_count(&self) -> u32; }

struct DieselEngine;
impl DieselEngine { fn start(&self) -> String { "diesel vroom".into() } }

struct Car {
    engine: DieselEngine,
    seats: u32,
}

impl HasEngine for Car { fn start(&self) -> String { self.engine.start() } }
impl HasSeats  for Car { fn seat_count(&self) -> u32 { self.seats } }
```

### Strategy Pattern (Runtime-Swappable Behavior)
```rust
trait PricingStrategy {
    fn calculate(&self, hours: u32) -> u32;
}

struct HourlyRate { rate: u32 }
struct Membership { monthly_fee: u32 }

impl PricingStrategy for HourlyRate {
    fn calculate(&self, h: u32) -> u32 { h * self.rate }
}
impl PricingStrategy for Membership {
    fn calculate(&self, _: u32) -> u32 { self.monthly_fee }
}

struct ParkingSession {
    strategy: Box<dyn PricingStrategy>,
}

impl ParkingSession {
    fn charge(&self, hours: u32) -> u32 {
        self.strategy.calculate(hours)
    }
}

fn main() {
    let session = ParkingSession {
        strategy: Box::new(HourlyRate { rate: 10 }),
    };
    println!("Cost: {}", session.charge(3)); // 30
}
```

---

## Blanket Implementations

Implement a trait for *any type* satisfying certain bounds.

```rust
trait Printable { fn print(&self); }

impl<T: std::fmt::Display> Printable for T {
    fn print(&self) { println!("{self}"); }
}

fn main() {
    42.print();
    "hello".print();
    3.14f64.print();
}
```

Used heavily in std — e.g., `impl<T: Display> ToString for T`.

---

## Trait Inheritance

```rust
trait Animal {
    fn name(&self) -> &str;
}

trait DomesticAnimal: Animal { // DomesticAnimal requires Animal
    fn owner(&self) -> &str;
}

struct Dog { name: String, owner: String }

impl Animal         for Dog { fn name(&self)  -> &str { &self.name  } }
impl DomesticAnimal for Dog { fn owner(&self) -> &str { &self.owner } }
```

Not real inheritance — just a constraint that any type implementing `DomesticAnimal` must also implement `Animal`.

---

## Rust vs Java Safety Differences

| Java | Rust equivalent |
|------|-----------------|
| `null` | `Option<T>` — compiler forces handling |
| unchecked exceptions | `Result<T, E>` — errors are explicit |
| shared mutable state (free) | `Arc<Mutex<T>>` — synchronized explicitly |
| inheritance for polymorphism | traits + composition |
| `instanceof` + cast | pattern matching on enums/trait objects |
