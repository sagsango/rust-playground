



use::std::collections::VecDeque;

/*
    We dont have stack & queue in Rust:std (may be there is some library)
    So we will use std::collections::VecDeque
*/

fn main() {
    println!("Hello, world!");

    let mut stack: VecDeque<i32> = VecDeque::new();
    let mut queue: VecDeque<i32> = VecDeque::new();


    for i in 0..10 {
        stack.push_back(i);
        queue.push_back(i);
    }


    loop {
        match stack.pop_back() {
            Some(i) => {
                println!("stack Pop:{}", i);
            }
            _ => {
                break;
            }
        }
    }

    loop {
        match queue.pop_front() {
            Some(i) => {
                println!("queue Pop:{}", i);
            }
            _ => {
                break;
            }
        }
    }
}
