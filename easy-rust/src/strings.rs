pub(crate) fn strings() {
    //String -> global String , owned type
    let my_name = "Jimin"; // -> &str
    my_name.to_string();
    let mut my_name_string = String::from("Jimin");
    my_name_string.push_str("3!");
    println!("My name is {}", my_name_string);
    //String type -> sized type; -> Heap
    //&str type -> dynamic type; -> Stack
    let mut capacity_string = String::with_capacity(26); // this method is init for capacity size,
}