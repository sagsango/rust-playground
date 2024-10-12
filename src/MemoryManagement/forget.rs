//! This file is a part of the `Memory Management` project.
//! Which explains the use of `Drop` in case of `Arc` and `Mutex`.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/*
forget(object) :
                1. Now when the this object goes out of scope, the refcount wont be decremented. 
                2. The object refcount remains the same.
                3. If object = Arc::clone(&parent_object), when parent_object goes out of scope, the refcount will be decremented by 1.
                4. When refcount becomes 0 then only drop() will be called (object will be dropped).
                
                
Initial Arc count: 1
Initial Arc count: 2
Initial Arc count: 2
Before forgetting: Arc count: 2
Arc object forgotten!
Now Parent continues Arc count: 2
Initial Arc count: 3
Initial Arc count: 3
Before setting to None: Arc count: 3
Shared data set to None!
Arc count after setting to None: 3
Final Arc count: 2
*/


pub fn test_forget_01() {
    // Create a shared Arc object with some initial data
    let shared_data = Arc::new(Mutex::new(Some(42))); // Wrap in Option

    // Print the initial strong count
    println!("Initial Arc count: {}", Arc::strong_count(&shared_data));

    let data_clone = Arc::clone(&shared_data);
    
    println!("Initial Arc count: {}", Arc::strong_count(&shared_data));
    println!("Initial Arc count: {}", Arc::strong_count(&data_clone));
     
    // Spawn a new thread to forget the Arc object after some time
    let _ = thread::spawn(move || {
        // Wait for 3 seconds before forgetting the Arc
        thread::sleep(Duration::from_secs(3));
        
        // Print the Arc count before forgetting
        println!("Before forgetting: Arc count: {}", Arc::strong_count(&data_clone));
        
        // Forget the Arc object (this will prevent its automatic dropping)
        std::mem::forget(data_clone);

        println!("Arc object forgotten!");
       
    }).join();
    
    println!("Now Parent continues Arc count: {}", Arc::strong_count(&shared_data));

    // Now, let's simulate making the shared data NONE after some time
    let data_clone2 = Arc::clone(&shared_data);
    
    println!("Initial Arc count: {}", Arc::strong_count(&shared_data));
    println!("Initial Arc count: {}", Arc::strong_count(&data_clone2));
    let _ = thread::spawn(move || {
        // Wait for 5 seconds before setting the shared data to None
        thread::sleep(Duration::from_secs(5));
        
        // Print the Arc count before setting to None
        println!("Before setting to None: Arc count: {}", Arc::strong_count(&data_clone2));
        

        // Lock the mutex and set the data to None
        let mut data = data_clone2.lock().unwrap();
        *data = None; // Set to None

        println!("Shared data set to None!");
        // Print the Arc count after setting to None
        println!("Arc count after setting to None: {}", Arc::strong_count(&data_clone2));
    }).join();

    // Wait to see the output from threads
    thread::sleep(Duration::from_secs(10));
    
    // Print final count before main exits
    println!("Final Arc count: {}", Arc::strong_count(&shared_data));
}



/*
Setting Data to None results the call of  drop() function of the struct.
Even the object is dropped, the refcount will still not zero.

Initial Arc count: 1
After cloning: Arc count: 2
Before forgetting: Arc count: 2
Arc object forgotten!
Parent continues: Arc count: 2
After second clone: Arc count: 3
Before setting to None: Arc count: 3
Dropping YourStruct with value: 42---------------------------------------------->>> DROPPED!!
Shared data set to None!
Arc count after setting to None: 3
Final Arc count: 2
*/

fn test_forget_02() {
    // Define a custom struct that implements Drop
    #[derive(Debug)]
    struct YourStruct {
        value: i32,
    }

    impl YourStruct {
        fn new(value: i32) -> YourStruct {
            YourStruct { value }
        }
    }

    // Implement Drop for the struct to observe when it gets dropped
    impl Drop for YourStruct {
        fn drop(&mut self) {
            println!("Dropping YourStruct with value: {}---------------------------------------------->>> DROPPED!!", self.value);
        }
    }

    // Create a shared Arc object wrapping YourStruct and put it inside Option for setting None later
    let shared_data = Arc::new(Mutex::new(Some(YourStruct::new(42)))); 

    // Print the initial strong count
    println!("Initial Arc count: {}", Arc::strong_count(&shared_data));

    // Create a clone of the Arc
    let data_clone = Arc::clone(&shared_data);
    
    println!("After cloning: Arc count: {}", Arc::strong_count(&shared_data));
     
    // Spawn a new thread to forget the Arc object after some time
    let _ = thread::spawn(move || {
        // Wait for 3 seconds before forgetting the Arc
        thread::sleep(Duration::from_secs(3));
        
        // Print the Arc count before forgetting
        println!("Before forgetting: Arc count: {}", Arc::strong_count(&data_clone));
        
        // Forget the Arc object (this will prevent its automatic dropping)
        std::mem::forget(data_clone);
        
        println!("Arc object forgotten!");
    }).join();
    
    println!("Parent continues: Arc count: {}", Arc::strong_count(&shared_data));

    // Now, let's simulate making the shared data None after some time
    let data_clone2 = Arc::clone(&shared_data);
    
    println!("After second clone: Arc count: {}", Arc::strong_count(&shared_data));
    let _ = thread::spawn(move || {
        // Wait for 5 seconds before setting the shared data to None
        thread::sleep(Duration::from_secs(5));
        
        // Print the Arc count before setting to None
        println!("Before setting to None: Arc count: {}", Arc::strong_count(&data_clone2));
        
        // Lock the mutex and set the data to None
        let mut data = data_clone2.lock().unwrap();
        *data = None; // Set to None
        
        println!("Shared data set to None!");
        println!("Arc count after setting to None: {}", Arc::strong_count(&data_clone2));
    }).join();

    // Wait to see the output from threads
    thread::sleep(Duration::from_secs(10));
    
    // Print final count before main exits
    println!("Final Arc count: {}", Arc::strong_count(&shared_data));
    // Print the value of the shared data
    println!("Shared data: {:?}", shared_data.lock().unwrap());
}


pub fn test_forget() {
    test_forget_01();
    test_forget_02();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_forget() {
        test_forget_02();
    }
}



