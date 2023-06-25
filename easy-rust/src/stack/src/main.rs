use std::arch::aarch64::int32x2_t;
use std::io::Read;

fn main() {
    let mut line = String::new();
    let inputSize = std::io::stdin().read_line(&mut line).unwrap().to_string();
    let inputSize = inputSize.parse().unwrap();
    let mut my_stack = Stack::new();
    for number in 0..inputSize {
        println!("number {}", number);
    }
    my_stack.push(1);
    my_stack.push(2);
    println!("{:?}", my_stack.top());
    println!("{:?}", my_stack.size());
    println!("{:?}", my_stack.empty());
    println!("{:?}", my_stack.pop());
    println!("{:?}", my_stack.pop());
    println!("{:?}", my_stack.pop());
    println!("{:?}", my_stack.size());
    println!("{:?}", my_stack.empty());
    println!("{:?}", my_stack.pop());
    my_stack.push(3);
    println!("{:?}", my_stack.empty());
    println!("{:?}", my_stack.pop());
}

struct Stack<T: PartialEq> {
    valueList: Vec<T>,
}

impl Stack<i32> {
    fn new() -> Stack<i32> {
        Self {
            valueList: vec![],
        }
    }

    fn push(&mut self, input: i32) {
        self.valueList.push(input)
    }

    fn pop(&mut self) -> i32 {
        match self.valueList.pop() {
            None => -1,
            Some(value) => value
        }
    }

    fn size(&self) -> i32 {
        self.valueList.len() as i32
    }

    fn empty(&self) -> i32 {
        if self.valueList.is_empty() {
            1
        } else {
            0
        }
    }
    fn top(&self) -> i32 {
        let index = self.valueList.len() - 1;
        let result = match self.valueList.get(index) {
            Some(value) => Ok(*value),
            None => Err(-1),
        };
        result.unwrap()
    }
}
