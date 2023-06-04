pub(crate) fn uninitialized_variable() {
    let my_number: u8;
    //it's not working because you never use a uninitialized variable in rust
    // println!("{}",my_number);

    {
        my_number = 8;
    }
    println!("{}",my_number);
}