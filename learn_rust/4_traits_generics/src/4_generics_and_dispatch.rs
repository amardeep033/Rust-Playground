// Q: Predict — a function that returns `impl Area` and does `if flag { Circle } else {
//    Square }`. Does it compile?

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

// fn make(flag: bool) -> impl Area {
//     if flag { Circle { r: 1.0 } } else { Square { s: 2.0 } } // ❌ if/else return DIFFERENT types
// }
fn make(flag: bool) -> Box<dyn Area> {
    // ✅ dyn erases the concrete type
    if flag { Box::new(Circle { r: 1.0 }) } else { Box::new(Square { s: 2.0 }) }
}

fn print_static<T: Area>(shape: &T) {
    println!("static  {:.2}", shape.area()); // monomorphized per type
}

fn main() {
    print_static(&Circle { r: 2.0 }); // compiler stamps out a Circle-specific copy
    print_static(&Square { s: 3.0 }); // ...and a Square-specific copy
    println!("dyn     {:.2}", make(true).area()); // one function, vtable lookup at runtime
}

// A: The `impl Area` version does NOT compile — `impl Trait` in return position is ONE
//    hidden concrete type, but the branches return Circle vs Square. `Box<dyn Area>` works
//    because `dyn` erases the concrete type behind a vtable, so both branches unify. Static
//    dispatch = zero-cost but one type per instantiation; dynamic = one flexible copy + a
//    pointer indirection.
//
// ── more Q&A ──
// Q: What does "monomorphization" cost?
// A: The compiler generates a separate specialized copy of the generic for EACH concrete
//    type → fast (inlinable, no indirection) but grows the binary ("code bloat"). `dyn`
//    trades that for one copy + a runtime vtable hop.
// Q: When must you use `dyn` (generics won't do)?
// A: Heterogeneous collections (`Vec<Box<dyn Area>>`), returning different types from one
//    fn, or a type chosen at runtime. Generics fix ONE type per call site, so they can't hold a mix.
