// Slices is primitive types, so you use a reference &array
// it looks likes &str and String

pub(crate) fn slices() {
    let season = ["spring", "summer", "autumn", "winter"];
    println!("{:?}", &season[0..2]);
    println!("{:?}", &season[..]);
    println!("{:?}", &season[..=2]);
}