use std::collections::VecDeque;


///
/// VecDeque is a double-ended queue implemented with a growable ring buffer.
/// It is a linear data structure.
/// It is a growable, mutable, owned, UTF-8 encoded string.
/// We are using std::collections::VecDeque
/// 
pub fn test() {
    /* XXX: VecDeque */
    let mut v = std::collections::VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_back(4);
    v.push_back(5);
    for i in &v {
        println!("Value: {}", i);
    }

    v.pop_back();
    v.push_front(0);
    v.push_back(6);
    for i in &v {
        println!("Value: {}", i);
    }
}