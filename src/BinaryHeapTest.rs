use std::collections::BinaryHeap;
pub fn test() {
    /* XXX: BinaryHeap */
    let mut b = std::collections::BinaryHeap::new();
    b.push(1);
    b.push(2);
    b.push(3);
    b.push(4);
    b.push(5);
    for i in &b {
        println!("Value: {}", i);
    }
    let val = b.peek();
    println!("Value: {:?}", val);
    let val = b.pop();
    println!("Value: {:?}", val);
    for i in &b {
        println!("Value: {}", i);
    }
}