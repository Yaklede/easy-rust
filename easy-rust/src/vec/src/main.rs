fn main() {
    let name = &String::from("John");
    let name2 = &String::from("Jane");

    let mut vec = Vec::new();
    vec.push(name);
    vec.push(name2);

    let my_vec =  vec![name, name2];

    println!("this is vec =  {:?}", vec);
    println!("this is my_vec =  {:?}", my_vec);
}
