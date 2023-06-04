use crate::for_loop::for_loop;
use crate::uninitialized_variable::uninitialized_variable;

mod for_loop;
mod uninitialized_variable;
fn main() {
    uninitialized_variable();
    for_loop();
}
