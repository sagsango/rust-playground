
use std::collections::BTreeMap;

pub fn test() {
    /* XXX: BTreeMap */
    let mut b = std::collections::BTreeMap::new();
    b.insert(1, "One");
    b.insert(2, "Two");
    b.insert(3, "Three");
    b.insert(4, "Four");
    b.insert(5, "Five");
    b.insert (5, "Five__");
    for (key, value) in &b {
        println!("Key: {}, Value: {}", key, value);
    }   
    let val = match b.get(&3) {
        Some(val) => val,
        None => &"Not found",
    };
    println!("Value: {}", val);
}
