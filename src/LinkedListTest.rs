use std::collections::LinkedList;
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

