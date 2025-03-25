use std::sync::Arc;
use std::mem;

///
/// HashMap is a collection of key-value pairs.
/// It is a non-linear data structure.
/// It is a hash table based implementation.
/// Time complexity is O(1) for insert, delete, search.
/// We are using std::collections::HashMap
/// Check the library implementation for more details, in case of collisions.
/// 
pub fn test() {
    /* XXX: HashMap*/
    let shared_message  = Arc::new(String::from("Hello, World!"));
    let mut hashmap:std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    let message = shared_message.clone().to_string();
    let insert = hashmap.insert(1, message);
    match insert {
        Some(msg) => println!("inserted Message: {}", msg),
        None => println!("Message not found"),
    };

    hashmap.insert(2, String::from("Hello, Rust!"));
    println!("Size of hashmap: {}", mem::size_of_val(&hashmap));
    for (key, value) in &hashmap {
        println!("Key: {}, Value: {}", key, value);
    }

    let message = shared_message.clone().to_string();
    let handle = std::thread::spawn(move || {
        println!("Thread message: {}", message);
    });

    handle.join().unwrap();
    println!("{}", shared_message.clone().to_string());

    let msg = match hashmap.get(&1) {
        Some(msg) => msg,
        None => &String::from("Not found"),
    };
   println!("Message: {}", msg);


    /* How to create object with typename */
   let mut hashmap:std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    hashmap.insert(1, String::from("One"));
    hashmap.insert(2, String::from("Two"));
    hashmap.insert(3, String::from("Three"));
    hashmap.insert(4, String::from("Four"));
    let message = shared_message.clone().to_string();
    let insert = hashmap.insert(5, message);
    match insert {
        Some(msg) => println!("inserted Message: {}", msg),
        None => println!("Message not found"),
    };
    for (key, value) in &hashmap {
        println!("Key: {}, Value: {}", key, value);
    }

    let msg = match hashmap.get(&5) {
        Some(msg) => msg,
        None => &String::from("Not found"),
    };
    println!("Message: {}", msg);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashMap() {
        test();
    }
}
