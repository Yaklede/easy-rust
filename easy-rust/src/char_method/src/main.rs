// chars() - iterator of char
// .count() - counts number of items in iterator
// char_indices
// chars().enumerate()

fn main() {
    let big_string = "Hello there, I am a &str";

    println!("big string count {} chars", big_string.chars().count());
    println!("big string length is {}", big_string.len());

    big_string.char_indices().for_each(|(index, char)| {
        println!("index is {index} char is {char}")
    });

    let my_vec = vec![7,8,9];
    my_vec.iter().for_each(|number| println!("We don't care about the number {number}"));
}
