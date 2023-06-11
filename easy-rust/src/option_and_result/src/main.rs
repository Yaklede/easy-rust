// Option - Maybe there, maybe not
// Result - May not work
enum Option<T> {
    None,
    Some(T)
}

enum Result<T,E> {
    Ok(T),
    Err(E)
}

mod option;
mod result;
fn main() {
    // option::option()
    result::result()
}


