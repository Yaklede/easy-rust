pub(crate) fn number() {
    let my_number: u8 = 100; //u = unsigned , i = not unsigned , ex) u8 -> unsigned 8byte
    let my_other_number = 50;
    let my_three_number = my_number + my_other_number; // auto type casting

    println!("{}", my_three_number);

    let u16_number: u16 = 16;
    let u8_number: u8 = 8;
    let other_number = 8_u8;

    /**u8_number + u16_number; it's not working because it have to convert to u8 or u16 if you try sum other types it happens compile error **/
    let _mix_number = u8_number as u16 + u16_number;
    println!("{}", _mix_number);

    //float
    let my_number_float = 9.0; //default type = float64, sometimes you use type float32 but it happen light error
    let i32_number = 9;
    println!("{}", my_number_float as i32 + i32_number);
}