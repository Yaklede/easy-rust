// & immutable reference / shared reference
// &mut mutable reference / unique reference
// && **
pub(crate) fn mutable_reference() {
    let mut my_number = 0;
    let num_ref = &mut my_number;
    *num_ref = 10;
    println!("Number refer first = {}", my_number);
    let num_ref_two =  &mut &mut my_number;
    **num_ref_two = 11;
    println!("Number refer two {}", my_number);
}