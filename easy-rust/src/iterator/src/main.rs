fn main() {
    let num_vec = vec![2, 4, 6];

    let map = num_vec.iter()
        .enumerate()
        .map(|(index, number)| {
            println!("The number at index {index} is {number}")
        }).collect::<Vec<_>>();

    println!("{map:?}");

    zip();
}

// .zip
use std::collections::HashMap;

fn zip() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // Vec<&str>

    let number_word_hashmap = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect::<HashMap<_,_>>();

    number_word_hashmap.iter()
        .for_each(|(key, value)| {
        println!("key = {key}, value = {value}")
    });

    let result_str = number_word_hashmap.get(&10).unwrap_or_else(|| {
        println!("Help");
        &"no number"
    });

    println!("result = {result_str}")
}
