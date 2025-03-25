use std::collections::BTreeSet;

///
/// BTreeSet is a set based on a B-Tree.
/// Time complexity is O(log n) for insert, delete, search.
/// This is a sorted set.
/// we are using std::collections::BTreeSet
/// Every key is unique, and olny one copy of the key is stored.
/// 
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
