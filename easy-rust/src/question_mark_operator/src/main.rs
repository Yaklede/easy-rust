// The question mark operator
// ?
use std::num::ParseIntError;

// gives the error to the caller
// if you use a question mark for method you should using a return type <Result>
fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parse_number = input.parse::<i32>()?; //return Error
    Ok(parse_number)
}

fn main() {
    for item in vec!["Seven","8","9.0","nice", "7070"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
