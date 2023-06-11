fn main() {
    for_loop()
}

fn for_loop() {
    for number in 0..3 { // 0..3 exclusive Range, 0..=3 inclusive Range
        println!("now number is {}", number);
    }

    for _ in 0..3 {
        println!("I dont need a number");
    }

    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3{
            break counter;
        }
    };
    println!("my number is {}", my_number);
}

fn default_loop() {
    let mut counter = 0;
    let mut counter2 = 0;
    'first_loop: loop {
        counter += 1;
        println!("the counter is : {}", counter);
        if counter > 9 {
            println!("Now entering the second loop");

            'second_loop: loop{
                counter2 += 1;
                println!("counter2 is now {}", counter2);
                if counter2 == 3{
                    break 'first_loop;
                }
            }
        }
    }
}

