pub(crate) fn reference_and_shadowing() {
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("print number ref = {}", number_ref);

    //shadowing
    let country = "my country";
    let country_ref = &country;
    let country = 10;
    println!("ref = {} country = {}", country_ref, country)
}