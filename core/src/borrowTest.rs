// This file contains the code for the borrow test
fn mut_borrow() {
    let mut x = 10;
    let y = &mut x;
    *y += 1;

    // cannot borrow `x` as mutable more than once at a time
    /*
    let z = &mut x;
    *z += 1;
    print!("Value: {}", z);
    */

    println!("Value: {}", y);
}

fn immut_borrow() {
    let x = 10;
    let y = &x;
    let z = &x;
    let w = &x;
    println!("Value: {}", y);
    println!("Value: {}", z);
    println!("Value: {}", w);
}

/*
    Borrow Rule (for primitive & collection):
    1. You can have either one mutable reference or multiple immutable references, but not both at the same time.
    2. References must always be valid.
    3. You can't borrow a mutable reference while an immutable reference is in scope.
    4. You can't borrow an immutable reference while a mutable reference is in scope.
    5. You can't borrow a mutable reference more than once.

*/
fn multiple_borrow_primitive() {
    let mut x = 32;
    let y = &x;
    let z = &x;
   
    let w = &y;
  
    // This will cause an error
    /*
    let p = &mut x;
    *p += 1;
    println!("Value: {}", p);
    */

    println!("Value: {}", y);
    println!("Value: {}", z);
    println!("Value: {}", w);
}

fn multiple_borrow_collection() {
    let mut x = vec![1, 2, 3, 4, 5];
    let y = &x;
    let z = &x;
   
    let w = &y;
    
    // This will cause an error
    /*
    let p = &mut x;
    p.push(6);
    println!("Value: {:?}", p);
    */

    println!("Value: {:?}", y);
    println!("Value: {:?}", z);
    println!("Value: {:?}", w);
}

///
/// Test the borrow functionality
/// We are testing the following:
/// - mut_borrow
/// - immut_borrow
/// - multiple_borrow_primitive
/// - multiple_borrow_collection
/// 
pub fn test() {
    mut_borrow();
    immut_borrow();
    multiple_borrow_primitive();
    multiple_borrow_collection();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_borrow() {
        mut_borrow();
    }

    #[test]
    fn test_immut_borrow() {
        immut_borrow();
    }

    #[test]
    fn test_multiple_borrow() {
        test();
    }
}