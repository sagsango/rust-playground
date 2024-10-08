use std::collections::HashSet;
use std::mem;

///
/// HashSet is a set based on a hash table.
/// Time complexity is O(1) for insert, delete, search.
/// we are using std::collections::HashSet
/// Every key is unique, and olny one copy of the key is stored.
/// 
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

