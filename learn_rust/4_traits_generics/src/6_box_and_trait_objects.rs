// Q: Predict — `Vec<dyn Animal>` won't compile but `Vec<Box<dyn Animal>>` will. Both hold
//    "any Animal", so why does one work?

trait Animal {
    fn speak(&self) -> String;
}
struct Dog; // 0 bytes
struct Cat {
    name: String,
} // 24 bytes
impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".into()
    }
}
impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} meow", self.name)
    }
}

// recursive type needs Box too:
enum List {
    Cons(i32, Box<List>), // ❌ without Box: infinite size (List contains List contains ...)
    Nil,
}

fn main() {
    // let zoo: Vec<dyn Animal> = ...;             // ❌ dyn Animal is unsized → can't be a Vec element
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat { name: "Tom".into() })];
    for a in &zoo {
        println!("{}", a.speak());
    }

    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    let mut node = &list;
    while let Cons(v, next) = node {
        print!("{v} ");
        node = next;
    }
    println!();
}

// A: A Vec stores elements CONTIGUOUSLY, so every element must be the same known size. Dog
//    (0 bytes) and Cat (24 bytes) differ, and `dyn Animal` is unsized — so `Vec<dyn Animal>`
//    can't lay out its elements. `Box<dyn Animal>` makes each element a same-size fat pointer
//    (data ptr + vtable ptr), each pointing to its own value on the heap. Uniform size in
//    the Vec, varied size on the heap.
//
// ── more Q&A ──
// Q: Why does the recursive `List` need `Box`?
// A: Without it, `List` contains a `List` contains a `List`… → infinite size, the compiler
//    can't lay it out. `Box` is a fixed-size pointer, breaking the cycle (the next node lives
//    on the heap).
// Q: Why is there no `Vec<Box<dyn Clone>>` / `dyn Clone`?
// A: Object safety: `Clone::clone` returns `Self`, and a vtable needs ONE fixed return
//    layout. Methods returning `Self` or using generics can't go in a vtable → not `dyn`-able.
