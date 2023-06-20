fn main() {
    unsafe {
        raw_pointer();
    }
}

unsafe fn raw_pointer() {
    let value = 42;
    let address = &value as *const i32;
    let r = *address;
    println!("{:?} {:?}", address, r);
    let test = r;
    println!("test = {}", test);
}
