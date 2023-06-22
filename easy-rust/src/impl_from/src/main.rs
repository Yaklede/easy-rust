use std::fmt::Display;
use std::ptr::addr_of;


fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {
        print!("{item} ");
    }
    println!();
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>
}


impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population
        }
    }
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self {
            cities
        }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population)
        }
    }
}

fn main() {
    let busan = City::new("busan", 1_000_000);
    let seoul = City::new("seoul", 20_000_000);

    let korean_cities = vec![busan, seoul];

    let korea: Country = korean_cities.into();

    korea.print_cities()
}
