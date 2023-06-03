/// documentation type comments ex, (Book으로 쓰는 타입이다.)
struct Book;
pub(crate) fn comment() {
    //  ( // ) mean is normal type comments
    println!("Hello, world!");
    /*중간에 넣을 수 있는 주석*/
    let _my_number = 10; // if you insert _ front of variable , compiler dismiss in warning message
    println!(_my_number)
}