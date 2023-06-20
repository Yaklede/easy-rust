// traits // verbs / adjective
// it's very similar interface

use std::fmt::Display;

struct Animal {
    name: String
}

trait Canine {
    fn bark(&self) {
        println!("default method bark")
    }

    fn run(&self) {
        println!("default method run");
    }
}

impl Canine for Animal {
    fn bark(&self) {
        println!("{} mang mang", self.name)
    }
    fn run(&self) {
        println!("{} run mang",self.name)
    }
}

fn main() {
    let animal = Animal {
        name: "my_dog".to_string()
    };
    animal.run();
    animal.bark();
}
