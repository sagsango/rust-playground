use std::sync::Arc;
use std::mem;

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

}