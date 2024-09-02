use std::collections::BTreeSet;

pub fn test() {
    /* XXX: BTreeSet */
    let mut b = std::collections::BTreeSet::new();
    b.insert("One");
    b.insert("Two");
    b.insert("Three");
    b.insert("Four");
    b.insert("Five");

    let s = match b.get("One") {
        Some(s) => s,
        None => "Not found",
    };
    println!("BTreeSet: {}", s);
}
