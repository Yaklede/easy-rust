pub(crate) fn text() {
    let ASCII = 'A' as u8; //ASCII Code can convert to number type

    println!("ascii as number {}", ASCII);
    println!("test {}", std::mem::size_of::<char>());

    //string type is different char type, I think string is to compound char in rust
    //example
    let string = "test";
    println!("Slice is {} bytes and also {} charters.", string.len(), string.chars().count());
    let korean_string = "안녕";
    println!("Slice is {} bytes and also {} charters.", korean_string.len(), korean_string.chars().count());
    // string in the case of korean it makes one char and some bytes
}