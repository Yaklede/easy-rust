pub(crate) fn fancy_print() {
    //they are not using a \n method, when you start code you can see the this message in same line
    print!("This");
    print!("This");
    //raw print
    print!(r#"c\test\test"#);

    println!("Let me tell me\
    about anything stroy\
    ");

    //debug print
    let debug_value = 9;
    println!("{:?}", debug_value);

    //pointer print
    let my_value = &9;
    println!("{:p}", &my_value);

    //byte print, it makes number for binary number
    let my_byte_value = 9;
    println!("{:b}", my_byte_value);

    //combination print
    let title = "TODAY'S NEWS";
    println!("{:-^30}",title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let city = "SEOUL";
    let city2 = "TOKYO";
    println!("{city:-<15}{city2:->15}");
    //you should don't remember if you need to this command, you can find a std::fmt reference, link = https://doc.rust-lang.org/std/fmt/
}