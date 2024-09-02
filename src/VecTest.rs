use std::mem;
use std::vec::Vec;

pub fn test() {
    /* XXX: Vector */
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("Vector: {:?}", v);
    let val = match v.get(2) {
        Some(val) => val,
        None => &0,
    };
    println!("Value: {}", val);
    for i in &v {
        println!("Value: {}", i);
    }

    println!("Size of vector: {}", mem::size_of_val(&v));
    print!("Vector[0]: {}", v[0]);
}
