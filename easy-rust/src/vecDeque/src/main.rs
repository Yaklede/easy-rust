use std::collections::VecDeque;

fn main() {
    let mut my_vec = vec![0; 600_000];
    let mut vec_deque = VecDeque::from(vec![0; 600_000]);
    println!("{}", vec_deque.len());
    for i in 0..vec_deque.len() {
        vec_deque.remove(0);
        println!("my vec size {}", vec_deque.len());
    }
}
