use std::cell::RefCell;
use std::rc::Rc;

pub fn basic_test() {
    // Case 1: borrow() - Immutable borrow
    let data = RefCell::new(5);
    {
        let val = data.borrow();
        println!("Case 1 - Immutable Borrow: {}", *val);
    } // `val` goes out of scope here, so borrowing is released

    // Case 2: borrow_mut() - Mutable borrow
    {
        let mut val = data.borrow_mut();
        *val += 10;
        println!("Case 2 - Mutable Borrow: {}", *val);
    } // `val` goes out of scope, so mutable borrow is released

    // Case 3: replace() - Replaces the inner value and returns the old one
    let old_value = data.replace(100);
    println!("Case 3 - Replace: Old = {}, New = {}", old_value, data.borrow());

    // Case 4: replace_with() - Updates value based on the existing value
    //data.replace_with(|val| val * 2);
    //println!("Case 4 - Replace With: {}", data.borrow()); // Should be 200

    // Case 5: take() - Removes the value and replaces it with the default
    let taken_value = data.take();
    println!("Case 5 - Take: Taken = {}, New = {}", taken_value, data.borrow()); // New value should be 0

    // Case 6: get_mut() - Get mutable reference without runtime borrow check
    let mut data = RefCell::new(String::from("Hello"));
    data.get_mut().push_str(" World!");
    println!("Case 6 - Get Mut: {}", data.borrow());

    // Case 7: Using Rc<RefCell<T>> for shared mutability
    use std::rc::Rc;

    let shared_data = Rc::new(RefCell::new(10));

    let data_clone1 = Rc::clone(&shared_data);
    let data_clone2 = Rc::clone(&shared_data);

    {
        let mut val = data_clone1.borrow_mut();
        *val += 5;
        println!("Case 7 - Shared Mutability: {}", *val);
    }

    println!("Final Value in shared_data: {}", shared_data.borrow()); // Should be 15
}




#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
    }
}

fn option_test() {
    // Creating nodes
    let root = TreeNode::new(1);
    let left_child = TreeNode::new(2);
    let right_child = TreeNode::new(3);

    // Assign children using Option<Rc<RefCell<TreeNode>>>
    root.borrow_mut().left = Some(left_child.clone());
    root.borrow_mut().right = Some(right_child.clone());

    // Access left child's value using Option and Rc
    if let Some(left) = &root.borrow().left {
        println!("Left Child Value: {}", left.borrow().val);
    }

    // Modify the right child's value
    if let Some(right) = &root.borrow().right {
        right.borrow_mut().val = 42;
    }

    // Print entire tree
    println!("Updated Root Node: {:?}", root);
}

pub fn test() {
    basic_test();
    option_test();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        test();
    }
}
