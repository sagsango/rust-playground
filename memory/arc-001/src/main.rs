use std::sync::Arc;

fn main() {
    let x = Arc::new(10);
    let y = Arc::new(vec![1, 2, 3]);

    println!("Hello, world!");
}
