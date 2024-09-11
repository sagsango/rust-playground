//!
//!  This file demonstrate the use of Drop in case of Arc and Mutex
//! Lets put good examples and resource here
//! 

use std::sync::{Arc, Mutex};

struct MyStruct {
    data: i32,
}

impl Drop for MyStruct {
    /*
    ** Drop trait is used to define the behavior when the value goes out of scope
    ** This will be called only once, once refount goes to zero
    ** 
    ** It can be called 2 ways:
    ** 1. Explicitly calling drop() method
    ** 2. When the value goes out of scope
     */
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

fn drop_test_0() {
     // use multiple threads to demonstrate the drop order
    // and also print  the reference count
    let my_struct = Arc::new(Mutex::new(MyStruct { data: 42 }));
    let my_struct_clone = Arc::clone(&my_struct);
    // 20 threads save handle and then join
    let mut handles = Vec::new();

    drop(my_struct.clone());

    for _ in 0..20 {
        let my_struct_clone = Arc::clone(&my_struct);
        let handle = std::thread::spawn(move || {
            let my_struct = my_struct_clone.lock().unwrap();
            // printing thread id

            // sleep for 1 second so that main thread will be able to count correct refcount
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread id: {:?}", std::thread::current().id());
            // printing data
            println!("Data: {}", my_struct.data);
            // printing refcount
            println!("Reference count: {}", Arc::strong_count(&my_struct_clone));
        });
        handles.push(handle);
    }
    // print refcount; it should be 22; as all the threads are holding the reference and sleep for 1 second
    // We must have used barrier to make sure that all threads are holding the reference
    println!("====> Reference count: {}", Arc::strong_count(&my_struct_clone));
    println!("====> Reference count: {}", Arc::strong_count(&my_struct));

    // join all threads
    for handle in handles {
        handle.join().unwrap();
    }


    // drop the original Arc
    drop(my_struct_clone.clone());
    drop(my_struct_clone);
    drop(my_struct); // Here actually refcount goes to zero; so drop() will be called
}

fn main() {
   drop_test_0();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop() {
       drop_test_0
    }
}


