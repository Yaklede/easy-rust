fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

//more better result
fn check_if_five(number: i32) -> Result<i32,String> {
    match number {
        5 => Ok(number),
        _ =>  Err("Sorry, the number wasn't five".to_string())
    }
}

fn parse_number(number: &str) -> Result<i32, std::num::ParseIntError> {
    number.parse()
}

pub fn result() {
    // let result = check_error(6);
    // if result.is_ok() {
    //     println!("it's okay guys");
    // } else {
    //     println!("it's too bad guys");
    // }
    // let mut result_vec = Vec::new();
    //
    // for number in 0..=7 {
    //     result_vec.push(check_if_five(number))
    // }
    //
    // println!("{:#?}", result_vec);
    let number = parse_number("test");
    println!("{:?}", number);
    let mut result_vec = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("it's not number"));
    result_vec.push(parse_number("8"));

    for number in result_vec {
        println!("{:?}", number)
    }
}

