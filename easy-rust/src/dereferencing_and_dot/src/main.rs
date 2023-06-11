//references and the dot operator

struct Item {
    number: u8
}

// dot. operator
impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are they equal? {}", self.number == other_number);
    }
}


// Deref *
// if you want to more information you can find it in https://doc.rust-lang.org/stable/nomicon/dot-operator.html
fn main() {
    // let my_number = 10;
    // let reference = &my_number;
    //
    // println!("Are they the same? {}", my_number == reference); //it happens compile error because it's not same types

    let item = Item {
        number: 10
    };

    let item_reference = &item;
    let other_reference = &item_reference;

    item.compare_number(10);
    item_reference.compare_number(10); // &item
    other_reference.compare_number(10);
}
