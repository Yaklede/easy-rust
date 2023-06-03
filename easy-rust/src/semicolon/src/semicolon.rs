pub(crate) fn semicolon() {
    let tuple = empty_tuple();
    println!("{:?}", tuple);
    // if you using a println!({}) your not println empty_tuple, but you are using a println("{:?}") this command you can println
    // because when you are using a :? command compiler support for you
}

//rust () <- empty tuple type
fn empty_tuple() -> () {
}

//if you don't use a return command you don't have to using a semicolon because semicolon means return to unit type
fn is_empty_tuple(one: i32, two: i32) {
    one * two;
}

fn is_not_empty_tuple(one: i32, two: i32) -> i32 {
    one * two
}

fn is_not_empty_tuple_2(one: i32, two: i32) -> i32 {
    return one * two;
}

