use std::panic::catch_unwind;

//impl block
#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat(String),
    Dog(String),
}

impl AnimalType {
    fn get_name(&self) {
        use AnimalType::*;
        match self {
            Cat(name) => println!("Cat name is {}", name),
            Dog(name) => println!("Dog name is {}", name),
        }
    }
}

impl Animal {
    fn new(animal_type: AnimalType, age: u8) -> Self { // Self = Animal , function signature
        Self {
            age,
            animal_type,
        }
    }

    fn new_cat(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Cat("default cat".to_string()),
        }
    }

    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog("default dog".to_string()),
        }
    }

    fn print(&self) {
        println!("{:?}", self);
    }

    fn get_age(&self) -> u8 {
        self.age
    }

    fn set_age(&mut self, new_age: u8) {
        self.age = new_age;
    }

    fn get_animal_type(&self) -> &AnimalType {
        &self.animal_type
    }

    fn set_animal_type(&mut self, new_animal_type: AnimalType) {
        self.animal_type = new_animal_type;
    }
}

fn main() {
    let my_vec = vec![1, 2, 3];
    println!(" The Vec length is {}", my_vec.len());
    let mut my_animal = Animal::new(AnimalType::Cat("mew".to_string()), 5);
    println!("{:?}", my_animal);
    my_animal.print(); // dot operator, syntactic sugar
    //my_animal.print() == Animal::print(&my_animal);
    Animal::print(&my_animal); // method call

    my_animal.set_age(100);
    println!("{:?}", my_animal);

    my_animal.animal_type.get_name();
}
