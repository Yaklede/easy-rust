//OWNERSHIP

use std::thread::current;

fn print_country(country_name: String) {
    println!("country is {}",country_name);
}

fn borrow_print_country(country_name: &String) {
    println!("borrow country is {}", country_name);
}

pub(crate) fn reference_in_function() {
    let country = "korea".to_string();
    borrow_print_country(&country);
    print_country(country); // working
    //print_country(country); // not working because it already move to ownership
}