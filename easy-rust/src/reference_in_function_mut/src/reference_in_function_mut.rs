fn add_is_great(country_name: &mut String) {
    country_name.push_str("is great!");
    println!("Now it says: {}",country_name);
}

fn add_is_great_not_refer(mut country_name: String) { //take by value, declare as mutable
    country_name.push_str("is great!");
    println!("Now it says: {}",country_name);
}

pub (crate) fn reference_in_function_mut() {
    let mut my_country = "canada".to_string();
    add_is_great(&mut my_country);
    add_is_great(&mut my_country);
    println!("current value is {}", my_country);

    let korea = "korea".to_string();
    add_is_great_not_refer(korea);
}