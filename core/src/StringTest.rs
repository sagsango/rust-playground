use std::mem;
use std::string::String;


///
/// String is a collection of characters.
/// It is a linear data structure.
/// It is a growable, mutable, owned, UTF-8 encoded string.
/// We are using std::string::String
/// Strings are heap allocated.
/// 
/// Drop & Copy & Clone trait:
/// heap allocated so Drop trait is implemented for String.
/// So we can not implement Copy trait for String.
/// But we can use clone() for String because rust already implemented Clone trait for String.
/// 
fn general_test() {
    /* XXX: String */
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    println!("String: {}", s);
    let s1 = s.clone();
    println!("String1: {}", s1);
    println!("String: {}", s);

}

// impl Clone for String 
fn clone_for_string_test() {
    /*
        only traits defined in the current crate can be implemented for types defined outside of the crate
        define and implement a trait or new type insteadrustcClick for full compiler diagnostic
    */
    /*
    impl Clone for String {
        fn clone(&self) -> Self {
            let mut s = String::new();
            for c in self.chars() {
                s.push(c);
            }
            s
        }
    }
    */

    let s = String::from("Hello, World!");
    let s1 = s.clone();
    println!("String: {}, {}", s, s1);
}

pub fn test() {
    general_test();
    clone_for_string_test();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        test();
    }

    #[test]
    fn test_general_test() {
        general_test();
    }

    #[test]
    fn test_clone_for_string_test() {
        clone_for_string_test();
    }
}

