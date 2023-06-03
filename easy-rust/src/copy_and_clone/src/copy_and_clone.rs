// Ownership and copy types
// it's trivial to copy the bytes
// stack == copy type
// heap == non copy type
fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_str(input: &str) {
    println!("{}",input);
}

fn prints_string(input: String) {
    println!("{}", input);
}
// copy = copy types
// clone = non-copy type
pub(crate) fn copy_and_clone() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "Austria".to_string();
    let my_country_str = "StrCountry";
    prints_string(my_country.clone());
    prints_string(my_country);

    prints_str(my_country_str);
    prints_str(my_country_str);
}
