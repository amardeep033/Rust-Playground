// Q: Static vs dynamic dispatch — what's the cost and the capability difference?

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}
struct Square {
    s: f64,
}
impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}
impl Area for Square {
    fn area(&self) -> f64 {
        self.s * self.s
    }
}

fn print_static<T: Area>(shape: &T) {
    println!("{:.2}", shape.area());
}
fn print_dyn(shape: &dyn Area) {
    println!("{:.2}", shape.area());
}

fn main() {
    print_static(&Circle { r: 2.0 });

    let shapes: Vec<Box<dyn Area>> =
        vec![Box::new(Circle { r: 1.0 }), Box::new(Square { s: 2.0 })];
    for s in &shapes {
        print_dyn(s.as_ref());
    }
}

// A: Static (`<T: Area>` / `impl Area`): the compiler monomorphizes a specialized copy
//    per concrete type — zero runtime overhead, but the binary grows and each call
//    site is fixed to one type. Dynamic (`dyn Area`): one shared version resolves the
//    method at RUNTIME through a vtable — one pointer of indirection, but it lets you
//    mix concrete types in a single collection.
//
// ── more Q&A ──
// Q: What is monomorphization?
// A: The compiler generating a separate specialized copy of a generic fn/type for each
//    concrete type it's used with — the mechanism behind static dispatch.
// Q: What does a `dyn Trait` reference contain in memory?
// A: A fat pointer: (pointer to the data, pointer to the vtable of method impls).
// Q: Can `-> impl Trait` return two different concrete types?
// A: No — it's ONE hidden concrete type. For branches returning different types, use
//    `Box<dyn Trait>`.
