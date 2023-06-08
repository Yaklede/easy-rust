//tuple possible has a many different types
fn main() {
    let random_tuple = ("hello", 5, 'c');
    println!("first tuple value is {}, second tuple value is {}, third tuple value is {}",
             random_tuple.0,
             random_tuple.1,
             random_tuple.2
    );

    let str_vec = vec!["hello", "world"];
    let str_vec_number = vec!["one", "two", "three"];
    let mix_vec = vec![("one",1),  ("two",2), ("three",3)];
    println!("{:?}", mix_vec);
    let (a,b,c) = (mix_vec[0], mix_vec[1], mix_vec[2]);
    println!("a key = {}, b key = {}, c key = {}, a value = {}, b value = {}, c value = {}", a.0, b.0, c.0,a.1, b.1, c.1);

    let (a,b) = (str_vec[0], str_vec[1]);
    println!("A = {}, B = {}", a, b);

    let (_,_,c) = (str_vec_number[0], str_vec_number[1], str_vec_number[2]); // if you don't want all the variable you use a _ keyword
    println!("C = {}", c);
}
fn returns_empty_tuple() -> () {}
