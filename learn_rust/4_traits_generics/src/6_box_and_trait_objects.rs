// Q: Why does `Vec<dyn Animal>` fail to compile while `Vec<Box<dyn Animal>>` works?

trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat {
    name: String,
}
impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".into()
    }
}
impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says meow", self.name)
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let zoo: Vec<Box<dyn Animal>> =
        vec![Box::new(Dog), Box::new(Cat { name: "Tom".into() })];
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

// A: A Vec stores elements contiguously, so every element needs the SAME known size —
//    but `dyn Animal` is unsized and Dog/Cat differ in size, so it can't be a Vec
//    element directly. Box<dyn Animal> makes each element a same-size fat pointer
//    (data ptr + vtable ptr) to a heap value.
//
// ── more Q&A ──
// Q: What else is Box used for besides trait objects?
// A: Recursive types (a pointer gives them a known size, breaking the infinite layout)
//    and moving large values to the heap.
// Q: What makes a trait "object-safe" (usable as dyn)?
// A: No method returns `Self` and no method is generic — a vtable needs one fixed
//    layout. That's why there's no `dyn Clone`.
// Q: &dyn Trait vs Box<dyn Trait>?
// A: &dyn borrows an existing value; Box<dyn> owns a heap-allocated one.
