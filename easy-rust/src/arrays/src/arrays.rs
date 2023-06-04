// Collection types
// Array []
//
pub(crate) fn arrays() {
    let array_number = [1, 2, 3, 5, 6];
    let array_string = ["One,two"];
    let array_string2 = ["One,two,Three"];

    println!("Is array the same as string2? {} ", array_string == array_string2);

    //if you want know this type you can use a not exists method,
    //array_number.alalala();

    let array_auto = [0; 640];
    println!("{:?}", array_auto);

    //if you use a not exists index it happens compile error,however you don't want compile error, you have to use a get method,
    //array.get(), method is safety compile because if you use a get method it not happens compile error just say None
    println!("array_number_first = {:?}", array_number[0]);
    println!("not exists array index = {:?}", array_number.get(10));
}