//immutable by default type -> let (immutable) , let mut (mutable)
//shadowing is same name using again
pub(crate) fn mutable() {
    let mut mutable_value = 10; // mutable
    let immutable_value = 10; // immutable
    println!("this is immutable value = {}", immutable_value);
    println!("this is mutable value = {}", mutable_value + 10);
    shadowing();
}

fn shadowing() {
    //it's shadowing
    let my_value = 10;
    println!("this is shadowing 1 = {}", my_value);

    let my_value = "1000";
    println!("this is shadowing 2 = {}", my_value);
}