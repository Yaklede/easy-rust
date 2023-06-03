// & = reference

pub(crate) fn ownership() {
    println!("Country is {}", return_it());
}

//it's not working because it can't return
fn return_it() -> &'static String {
    let country = String::from("korea");
    &country
}