use std::os::unix::raw::time_t;

pub(crate) fn function() {
    let my_number = give_number(9,8);
    println!("{my_number}");
    print_number(9,8);
    println!("this is code block code {}", code_block(1,2));
}

fn give_number(one: i32 , two: i32) -> i32 {
    one * two
}

fn print_number(one: i32, two: i32) {
    let multiple_number = one * two;
    println!("{multiple_number}")
}

fn code_block(one: i32, two: i32) -> i32 {
    let multiple_number = {
        let first_number = 10;
        first_number * one * two
    };
    multiple_number
}