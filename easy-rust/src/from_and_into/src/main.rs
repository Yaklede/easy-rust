// trait == superpower
// This type implements (trait name)
// From , Into
fn main() {
    let my_name = String::from("John");
    let my_city: String = "Seoul".into();

    println!("{}, {}", my_name, my_city);
    let my_vec = Vec::from([1,2,3]);

    println!("{:?}", my_vec);
}
