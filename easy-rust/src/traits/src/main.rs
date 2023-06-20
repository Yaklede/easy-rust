// traits // verbs / adjective
// it's very similar interface


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

use std::fmt::{Display, Formatter};

struct Cat {
    name: String,
    age: u8
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

fn main() {
    // let animal = Animal {
    //     name: "my_dog".to_string()
    // };
    // animal.run();
    // animal.bark();
    let my_cat = Cat {
        name: "cat".to_string(),
        age: 10
    };
    println!("{my_cat}")
}
