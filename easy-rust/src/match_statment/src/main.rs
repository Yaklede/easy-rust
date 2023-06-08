use std::mem::transmute;

fn main() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "warm") => println!("It's a nice day"),
        ("cloudy", "cold") => println!("It's a cold day"),
        ("cloudy", _) => println!("It's a cloudy day"),
        _ => println!("I don't know weather"),
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if married == false => println!("Not married children {}", children),
        (c, m) if c == 0 && m => println!("Married but no children"),
        _ => println!("Married with children"),
    }

    let number = 5;
    // let some_variable = if number == 10 { 8 } else { "Something else" }; it's not working because they are different types
    let some_variable = if number == 10 { 8 } else { 9 };
    more_match();
    match_number(number);
}

fn more_match() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    let array = vec![first, second, third];
    for value in array {
        match_colors(value);
    }
}

fn match_colors(rgb: (u8, u8, u8)) {
    match rgb {
        (r, _, _) if r < 10 => println!("it's not red color"),
        (_, g, _) if g < 10 => println!("it's not green color"),
        (_, _, b) if b < 10 => println!("it's not blue color"),
        (_, _, _) => println!("All colors"),
    }
}

fn match_number(input: i32) {
    match input {
        number @ 0..=10 => println!("0-10, it's value is = {}", number),
        _ => println!("other"),
    }
}