use std::collections::LinkedList;

///
/// LinkedList is a doubly linked list.
/// Time complexity is O(1) for insert, delete, search.
/// we are using std::collections::LinkedList
/// There are many operations available in the library.
/// - push_back
/// - push_front
/// - pop_back
/// - pop_front
/// - append
/// - other operations
/// 
pub fn test() {
    /* XXX: LinkedList */
    let mut l = std::collections::LinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    l.push_back(4);
    l.push_back(5);
    for i in &l {
        println!("Value: {}", i);
    }
    let mut l2 = l.clone();
    l.pop_back();
    l.append(&mut l2);
    l.push_front(0);
    //l.reverse();
    for i in &l {
        println!("Value: {}", i);
    }
}

