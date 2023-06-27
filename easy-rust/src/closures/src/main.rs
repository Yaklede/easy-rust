// closures = anonymous functions that capture their environment
// for example , a|nonymous = no name , enclose
// || - pipes
fn main() {
    let my_number = 10;
    // Look at this, anonymous function and closures they are very close mean but anonymous function isn't having a argument
    // it's very different, if you see anonymous_function and closure, almost closure function
    // if you want more information you can search a documentation in https://doc.rust-lang.org/book/ch13-01-closures.html
    let anonymous_function = || println!("this is anonymous function");
    let my_closure = |x: i32| println!("Hello, world! {}", x + my_number);
    my_closure(9);

    //if you want to long closures function , you can follow it
    let my_long_closure = || {
        let number = 10;
        let other_number = 99;
        println!("The two numbers are {number} and {other_number}");
        number + other_number
    };

    let my_var = my_long_closure();
    println!("{my_var}");

    let my_vec = vec![1,2,3];

    let forth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });

    println!("{forth}");

    map_and_for_each();
}

//zero cost abstractions
fn zero_cost_abstraction() {
    let my_vec = vec![1,2,3];
    //it's zero cost abstraction , it's means if you use a function abstract function chain it has a same function speed when you use a app
    // my_vec.iter().filter().collect()
}

fn map_and_for_each() {
    let num_vec = vec![1,2,3];
    //map
    let double_vec = num_vec.iter().map(|number| number * 2).collect::<Vec<i32>>();

    println!("{double_vec:?}");

    //foreach
    // num_vec.iter().enumerate().for_each(|(index, item) | println!(" this index is {index:?} and value is {item}"))

    //foreach with tuple
    num_vec.iter().enumerate().for_each(|tuple| println!("index is {} value is {}", tuple.0, tuple.1))
}

