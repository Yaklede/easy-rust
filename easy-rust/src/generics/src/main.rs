
// you can use any scope name for example
// fn give_thing<GenericType>(input: GenericType) -> GenericType
use std::fmt::Display;
use std::cmp::PartialOrd;

fn give_thing<T: Display>(input: T) -> T {
    println!("{}",input); // Display
    input
}
// generics <-> concrete
// "it's little generic"
fn main() {
    let x = give_thing(100);
    let y = give_thing(String::from("value"));
    println!("{}", x);
    println!("{}", y);

    compare_and_print("Listen up!", 9,8);
}
// fn compare_and_print<T: Display,U: Display + PartialOrd>(statement: T, num_1: U, num_2: U)
fn compare_and_print<T,U>(statement: T, num_1: U, num_2: U)
    where
    T: Display,
    U: Display + PartialOrd
{
    println!(
        "{} Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

