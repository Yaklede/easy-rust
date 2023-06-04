pub(crate) fn for_loop() {
    println!("{}",loop_then_return(0));
}

fn loop_then_return(mut count: i32) -> i32 {
    loop {
        count += 1;
        if count % 50 == 0 {
            break;
        }
    }
    count
}