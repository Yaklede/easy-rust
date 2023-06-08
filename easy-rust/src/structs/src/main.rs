// structs == and , enum == or
use std::mem::size_of_val;

// unit struct
struct FileDirectory; // if you don't use a bite them it's useful because it's don't have any byte

// trait
// tuple struct
#[derive(Debug)]
struct Color(u8, u8, u8);

// named struct
#[derive(Debug)]
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

struct Numbers {
    one: u8,
    two: u8,
    three: u8,
}

fn takes_file_directory(file_directory: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    takes_file_directory(x);
    println!("The size is {}", std::mem::size_of::<FileDirectory>());

    let my_color = Color(255, 0, 0);
    println!("The color is {} {} {}", my_color.0, my_color.1, my_color.2);
    println!("{:?}", my_color);

    let canada = Country {
        population: 35_000_000,
        capital: String::from("Ottawa"),
        leader_name: String::from("Justin Trudeau"),
    };

    println!("the population is {}", canada.population);
    println!("The capital is {}", canada.capital);
    println!("The leader name is {}", canada.leader_name);
    println!("The country is {:#?}", canada);

    let population = 35_000_000;
    let capital = String::from("Ottawa");
    let leader_name = String::from("Justin Trudeau");

    let canada2 = Country {
        population,
        capital,
        leader_name,
    };

    println!("Country is {} bytes", size_of_val(&canada2));

    let numbers = Numbers {
        one: 10,
        two: 20,
        three: 30,
    };

    println!("number size is {}", size_of_val(&numbers));
}
