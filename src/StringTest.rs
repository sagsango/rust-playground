use std::mem;
use std::string::String;


///
/// String is a collection of characters.
/// It is a linear data structure.
/// It is a growable, mutable, owned, UTF-8 encoded string.
/// We are using std::string::String
/// Strings are heap allocated.
/// 
pub fn test() {
    /* XXX: String */
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    println!("String: {}", s);
    let s1 = s.clone();
    println!("String1: {}", s1);
    println!("String: {}", s);

}
