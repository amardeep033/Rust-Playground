// `Animal` is the shared interface used by both concrete types below.
trait Animal {
    fn speak(&self);
}

// Each type has its own concrete layout in memory.
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Both types satisfy the same trait contract, but keep their own behavior.
impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name);
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says Meow!", self.name);
    }
}

// Static dispatch version:
// The compiler generates a specialized copy of this function for each concrete
// `T` used at call sites, such as `Dog` and `Cat`.
// fn make_speak<T: Animal>(animal: &T) {
//     animal.speak();
// }

// Dynamic dispatch version:
// `&dyn Animal` is a trait object. The concrete type is chosen at runtime
// through a vtable, which lets one function accept many implementors.
fn make_speak(animal: &dyn Animal) {
    animal.speak();
}

fn main() {
    let dog = Dog {
        name: "Buddy".to_string(),
    };

    let cat = Cat {
        name: "Kitty".to_string(),
    };

    // Both static and dynamic dispatch work for direct calls with one value at a
    // time. `Dog` and `Cat` are coerced to `&dyn Animal` for this function.
    make_speak(&dog);
    make_speak(&cat);

    // A heterogeneous collection needs dynamic dispatch because `Vec<T>` can
    // store only one concrete `T`. `Box<dyn Animal>` gives each value a uniform
    // pointer type while preserving its concrete behavior behind the trait.
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(dog));
    animals.push(Box::new(cat));

    // Dereference the `Box` to pass a `&dyn Animal` into `make_speak`.
    for animal in animals {
        make_speak(&*animal);
    }
}
