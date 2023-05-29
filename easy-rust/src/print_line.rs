// macro = code function that writes code
pub(crate) fn print_line() {
    println!("normal print");

    let my_name = "Jimin";
    let my_age = 23;
    println!("My name is {} and my age is {}", my_name, give_age());


    let my_city = "Seoul";
    let year = 2023;
    let population = 9_999_123;

    //default method
    println!("The city of {} in {} had a population of {}", my_city, year, population);

    //if you use some nickname you can try it
    println!("The city of {city} in {year} had a population of {population}",
        city = my_city,
        year = year,
        population = population
    );

    //if you use a number you can try it
    println!("The city of {0} in {1} had a population of {2}",
             city = my_city,
             year = year,
             population = population
    );

    //but Recent rust version just use a variable name, it's very similar kotlin ${} grammar
    println!("The city of {my_city} in {year} had a population of {population}");
}


fn give_age() -> i32 {
    23
}