// blanket trait implementations
// implementing a trait for every type that you want to have it

use std::fmt::{Debug, Display};

trait Prints: {

    fn print_by_self(&self) where Self: Debug {
        println!("{:?}", self)
    }

    fn prints_somethings_self(&self) where Self: Display {
        println!("I like to print things");
    }

    fn prints_somethings_not_self() {
        println!("I like to print things");
    }
}


fn print_string <T: Display + AsRef<str>> (input: T) {
    println!("{input}")
}

#[derive(Debug)]
struct Person;
struct Building;


impl <T> Prints for T {}

fn main() {
    let person = Person{};
    let build = Building;
    person.print_by_self();
    let x = String::from("x");
    x.print_by_self();
    print_string("a");
}
