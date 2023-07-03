use std::collections::HashMap;
use std::hash::Hash;

//Iterator = a collection of things that you can call .next() on
fn main() {
    // .iter() - Iterator of references &T
    // .iter_mut() - Iterator of mutable references &mut T
    // .into_iter() - consuming iterator
    let vector_1 = vec![1, 2, 3];
    let vector_a = vector_1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector_b: Vec<i32> = vector_1.into_iter().map(|x| x * 10).collect();

    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|num| *num += 100);

    println!("{vector_a:?}");
    println!("{vector_b:?}");
    println!("{vector2:?}");
    // println!("{vector_1:?}");
    what_is_assert_eq();

    let mut my_library = Library::new();
    my_library.add_book("book");
    my_library.add_book("other Book");

    println!("{my_library:?}");

    for item in my_library {
        println!("{item}")
    }
}

//it's very useful when you search the reference documentation
fn what_is_assert_eq() {
    let my_vec = vec!['a', 'b', '거'];
    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), None);
}
//associated = goes together, the type that goes together with the trait

#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string())
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book_title) => Some(book_title + "is found"),// String  + &str
            None => None
        }
    }
}


fn map() {
    let num_vec = vec![2, 4, 6];
    let map = num_vec.iter()
        .enumerate()
        .map(|(index, number)| {
            println!("The number at index {index} is {number}")
        }).collect::<Vec<_>>();
    println!("{map:?}");
    zip();
}

//zip
fn zip() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // Vec<&str>
    let number_word_hashmap = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect::<HashMap<_, _>>();

    number_word_hashmap
        .iter()
        .for_each(|(key, value)| {
            println!("key = {key}, value = {value}");
        });
    let result_str = number_word_hashmap.get(&10).unwrap_or_else(|| {
        println!("Help");
        &"no number"
    });
    println!("result = {result_str}");
}
