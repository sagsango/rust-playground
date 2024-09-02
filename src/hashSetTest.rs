use std::collections::HashSet;
use std::mem;

pub fn test() {
    /* XXX: HashSet */
    let mut h = HashSet::new();
    h.insert("Hello, World!");
    h.insert("Hello, Rust!");
    println!("Size of HashSet: {}", mem::size_of_val(&h));
    h.insert("value");
    let s = match h.get("value") {
        Some(s) => s,
        None => "Not found",
    };
    println!("HashSet: {}", s);
}

