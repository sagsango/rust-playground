use std::mem;
use std::string::String;

pub fn test() {
    /* XXX: String */
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    println!("String: {}", s);
    let s1 = s.clone();
    println!("String1: {}", s1);
    println!("String: {}", s);

}
