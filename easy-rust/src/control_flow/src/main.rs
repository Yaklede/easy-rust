// rust is expression-based language
fn main() {
    //you can this control flow with if statements but it's not useful
    let my_number = 5;
    if my_number == 7 {
        println!("it's seven!");
    } else if my_number == 6 {
        println!("it's six!");
    } else {
        println!("it's a different number")
    }

    //it's useful rust
    let number = 5;
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => { // _ it's means I don't care, anything else
            println!("other number")
        }
    }

    let second_number = match number {
        0 => 20,
        1 => 30,
        _ => 50
    };

    println!("second number is {}", second_number);
}
