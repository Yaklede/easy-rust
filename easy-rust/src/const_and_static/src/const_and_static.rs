//const
const HIGH_SCORE: i32 = 20; // const type usually Upper case, this is global value

//static
static mut LOW_SCORE: i32 = 0; // this is var unsafe, so you don't use a mutable for static, but you can use it, see a line 12
pub(crate) unsafe fn const_and_static() {
    print_high_score();
}

unsafe fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
    unsafe {LOW_SCORE = 1;}
}