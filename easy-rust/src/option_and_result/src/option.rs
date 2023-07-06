
use Option::*;
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
}
//warp in an Option
// .except
// is.some
// is.none

pub fn option() {
    let new_vec = vec![1, 2];
    let index = take_fifth(new_vec);
    println!("{:?}", index);
    // .unwrap() - take out what is inside
    // safety method
    match index {
        Some(number) => println!("I got a number : {}", number),
        None => println!("There was nothing inside")
    }

    if index.is_some() { // bool
        println!("I got a number : {}", index.unwrap())
    }

    let first_try = vec![Some("some"), None, Some("Som2"), Some("good")];
    let second_try = vec![Some("some"), None, None, Some("good")];
    let third_try = vec![None, None, Some("Som2"), Some("notGood")];

    for index in 0..first_try.len() {
        println!("{:?}", first_try[index].or(second_try[index].or(third_try[index])))
    }
}