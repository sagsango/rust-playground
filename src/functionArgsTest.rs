use std::string::String;
use std::vec::Vec;

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn pass_string(s: String) {
    println!("String: {}", s);
}

// String is a wrapper around Vec<u8>
// https://doc.rust-lang.org/std/string/struct.String.html
// str size is not known at compile time
// https://doc.rust-lang.org/std/primitive.str.html
fn pass_str(s: &str) {
    println!("String: {}", s);
}

// https://doc.rust-lang.org/std/primitive.slice.html
// unknown size at compile time
fn pass_slice(s: &[u8]) {
    println!("Slice: {:?}", s);
}

// known size at compile time
// https://doc.rust-lang.org/std/primitive.array.html
fn pass_array(s: [u8; 5]) {
    println!("Array: {:?}", s);
}

// https://doc.rust-lang.org/std/vec/struct.Vec.html
// vector is allocated on heap
// size is not known at compile time; it is dynamic
// But we can pass it to the function
// because Vec is a wrapper around Vec<u8>
fn pass_vector(s: Vec<u8>) {
    println!("Vector: {:?}", s);
}

/*
the size for values of type `str` cannot be known at compilation time
the trait `Sized` is not implemented for `str`rustcClick for full compiler diagnostic

fn pass_str_itself(s:str) {
    println!("String: {}", s);
}
*/

fn slice_ans_vector() {
    let s = String::from("Hello, World!");
    pass_string(s.clone());
    //println!("String: {}", s); // XXX: s is moved to pass_string function

    let s = String::from("Hello, World!");
    pass_str(&s);
    println!("String: {}", s);

    let s = String::from("Hello, World!");
    pass_slice(s.as_bytes());
    println!("String: {}", s);

    let s = String::from("Hello, World!");
    pass_array([1, 2, 3, 4, 5]);
    println!("String: {}", s);

    let s = String::from("Hello, World!");
    pass_vector(vec![1, 2, 3, 4, 5]);
    println!("String: {}", s);

    let s = "hello";
    //pass_str_itself(s); // XXX: str is not allowed

    /* String slices are immutable */
    let mut s = "hello";
    //s.push_str(", world!"); // XXX: str is not mutable
    println!("String: {}", s);

    //println!("String: {}", s); // XXX: s is moved to pass_string function
}

fn largest1(a: &mut String, b : &mut String) -> String {
    if a.len() > b.len() {
        a.to_string()
    } else {
        b.to_string()
    }
}
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn largest2<'a>(a: &'a mut String, b : &'a mut String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn largest3<'a>(a: &'a mut String, b : &'a mut String) -> &'a mut String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn lifetime_test1() {
    let mut s1 = String::from("Hello");
    let mut s2 = String::from("World!");
    let s3 = largest1(&mut s1, &mut s2);
    println!("String: {}", s3);

    let mut s1 = String::from("Hello");
    {
        let mut s2 = String::from("World!");
        let s3 = largest1(&mut s1, &mut s2);
        println!("String: {}", s3);
    }
}

fn lifetime_test2() {
    let mut s1 = String::from("Hello");
    let mut s2 = String::from("World!");
    let mut s3 = largest2(&mut s1, &mut s2);
    println!("String: {}", s3);

    let mut s1 = String::from("Hello");
    let s3: &String;
    {
        let mut s2 = String::from("World!");
        s3 = largest2(&mut s1, &mut s2);
        println!("String: {}", s3);
    }
}

fn lifetime_test3() {
    let mut s1 = String::from("Hello");
    let mut s2 = String::from("World!");
    let mut s3 = largest2(&mut s1, &mut s2);
    println!("String: {}", s3);

    let mut s1 = String::from("Hello");
    let s3: &mut String;
    {
        let mut s2 = String::from("World!");
        s3 = largest3(&mut s1, &mut s2);
        println!("String: {}", s3);
    }
}

// 2 different lifetimes
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// TODO: Fix this
/*
fn largest4<'a, 'b>(a: &'a mut String, b : &'b mut String) -> &'a mut String
{
     if a.len() > b.len() {
         a
     } else {
         b
     }
 }
 */

fn lifetime_test() {
    lifetime_test1();
    lifetime_test2();
    lifetime_test3();
}

pub fn test() {
    slice_ans_vector();
    lifetime_test();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_args() {
        lifetime_test()
    }

    #[test]
    fn test_slice_ans_vector() {
        slice_ans_vector();
    }
}