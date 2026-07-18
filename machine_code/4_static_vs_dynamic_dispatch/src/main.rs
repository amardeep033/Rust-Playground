// A trait
trait Animal {
    fn speak(&self);
}

// Two types
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Trait implementations
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

// // Generic function
// fn make_speak<T: Animal>(animal: &T) {
//     animal.speak();
// }

// Dynamic dispatch
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

    make_speak(&dog);
    make_speak(&cat);
}