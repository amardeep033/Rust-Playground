# Lifetimes

## Mental Model
Lifetimes are annotations that describe *how long references must remain valid*.
They don't extend or change how long data lives — they name relationships
so the compiler can verify you're not creating dangling references.

Key insight: `'a` means "this reference is valid for *at least* the duration `'a`."

Rule: no reference can outlive the data it points to.

---

## Why Lifetimes Exist

### What happens?
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```
**Compile error**: missing lifetime specifier.

The compiler sees: the return could be `x` or `y`. But `x` and `y` might have different lifetimes.
If the caller holds the return value past `x`'s lifetime, and we returned `x`, that's a dangling pointer.

**Fix**: tie the output lifetime to both inputs
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
This says: "the returned reference lives at least as long as the shorter of `x` and `y`."

---

## Lifetimes Don't Extend Data

### What happens?
```rust
struct Cache<'a> {
    value: &'a str,
}

fn main() {
    let cache;
    {
        let s = String::from("hello");
        cache = Cache { value: &s }; // s borrowed into cache
    } // s dropped here
    println!("{}", cache.value); // ?
}
```
**Compile error**: `s` does not live long enough.

The annotation `'a` just says "the struct can't outlive the reference it holds." It cannot make `s` live longer.

**Fix**: ensure the data outlives the struct
```rust
fn main() {
    let s = String::from("hello");
    let cache = Cache { value: &s }; // s outlives cache
    println!("{}", cache.value);
}
```

---

## Struct With Lifetime

```rust
struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let novel = String::from("long story here. Second sentence.");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { text: first_sentence };
    println!("{}", excerpt.text);
}
```

Rule: if a struct holds a reference, it needs a lifetime annotation. The struct instance can't outlive the referenced data.

---

## Lifetime Elision — When Annotations are Optional

Three rules the compiler applies automatically (you don't need to write `'a` in these cases):

**Rule 1**: Each reference parameter gets its own lifetime
```rust
fn foo(x: &str, y: &str)
// compiler sees: fn foo<'a, 'b>(x: &'a str, y: &'b str)
```

**Rule 2**: If exactly one input lifetime, it's used for all outputs
```rust
fn foo(x: &str) -> &str
// compiler sees: fn foo<'a>(x: &'a str) -> &'a str
```

**Rule 3**: If one parameter is `&self` or `&mut self`, its lifetime goes to all outputs
```rust
impl S {
    fn get(&self, key: &str) -> &str
    // output lifetime = &self's lifetime
}
```

### What happens?
```rust
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap()
}
```
**Compiles fine** — rule 2 applies (one input, output gets same lifetime).

```rust
fn pick(a: &str, b: &str) -> &str {
    a // error!
}
```
**Compile error** — two inputs, rule 2 doesn't apply. Compiler can't determine which input the output ties to.

---

## The `'static` Lifetime

```rust
let s: &'static str = "I live forever";
```
`'static` means the reference lives for the entire program duration.
String literals are `'static` — they're baked into the binary.

### Common misconception
"Just add `'static` to fix a lifetime error." This usually doesn't work and hides actual problems.

```rust
fn bad<'a>(x: &'a str) -> &'static str {
    x // compile error: x doesn't live for 'static
}
```

### Legitimate `'static` uses
```rust
fn error_message() -> &'static str { "something went wrong" }
fn register_handler(h: Box<dyn Fn() + 'static>) { ... } // thread-safe callbacks
```

---

## Lifetime in impl Block

```rust
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    fn current_slice(&self) -> &str {
        &self.input[self.pos..] // elision rule 3: output tied to &self
    }
}
```

---

## Block Scope Trap (This Is Valid)

```rust
fn main() {
    let s = String::from("hello");
    let r;
    {
        r = &s; // valid: s lives longer than the block
    }
    println!("{r}"); // fine
}
```

**Why valid**: lifetime of a reference depends on the lifetime of the *referenced value* (`s`), not on where the reference variable was assigned.

---

## Demo: When Do You Actually Need Annotations?

```rust
// Needs 'a: two inputs, one output — compiler can't infer which ties to output
fn pick<'a>(a: &'a str, _b: &str) -> &'a str { a }

// No annotation needed: returns owned data (no reference relationship)
fn combine(a: &str, b: &str) -> String { format!("{a}{b}") }

// No annotation needed: impl method, elision rule 3
impl Cache {
    fn get(&self) -> &str { &self.value }
}

// Needs 'a: struct holding a reference
struct Wrapper<'a> { data: &'a str }
```

---

## Common Interview Trap

```rust
fn main() {
    let result;
    {
        let s1 = String::from("long string");
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str()); // ?
    }
    println!("{result}");
}
```
**Compile error**: `s1` and `s2` don't live long enough. The lifetime `'a` in `longest` is the shorter of both — but `result` is used after both are dropped.

**Fix**: extend `s1` and `s2` to the same scope as `result`.
