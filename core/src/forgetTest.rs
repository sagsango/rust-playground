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
Dropping YourStruct with value: 42 ---------------------------------------------->>> DROPPED!!
Shared data set to None!
Arc count after setting to None: 3
Final Arc count: 2

NOTE:
When you do *data = None; in this context, assuming data_clone2 is an Arc<Mutex<Option<T>>>, you are dereferencing the MutexGuard (data) and setting the underlying value inside the Option<T> to None.

Here’s a step-by-step breakdown of what happens:

Cloning the Arc:
Arc::clone(&shared_data) creates another reference to the shared data, incrementing the Arc's strong reference count. Both shared_data and data_clone2 will point to the same underlying Arc<Mutex<Option<T>>>.
Locking the Mutex:
Inside the spawned thread, when you do let mut data = data_clone2.lock().unwrap();, you are acquiring a lock on the Mutex. This gives you exclusive access to the data wrapped inside the Mutex.
Setting to None:
By doing *data = None;, you are modifying the contents of the Option inside the Arc<Mutex<Option<T>>>. This means the inner data (of type T, if it exists) is dropped, and the value becomes None. However, this does not affect the Arc reference count, as it controls the ownership of the Arc, not the contents of the Mutex.
Dropping the Inner Data:
The inner value of T (if there was any) is dropped when you set *data = None;. If T is a complex type or owns other resources, its destructor is called.
Arc Count:
The Arc::strong_count(&data_clone2) will still reflect the reference count of the Arc, which includes all active references (like shared_data and data_clone2). This count doesn't change by modifying the contents inside the Arc. Only when the references themselves are dropped will the reference count decrease.
Hence, after setting the data to None, the Arc count will remain the same unless one of the Arc references is dropped.
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

/*
Initial Arc count: 1
After cloning: Arc count: 2
Before forgetting: Arc count: 2
Arc object forgotten!
Parent continues: Arc count: 2
After second clone: Arc count: 3
Before setting to None: Arc count: 3
Dropping MyEnum::Struct with value: YourStruct { value: 42 } >>> DROPPED!!
Dropping YourStruct with value: 42 >>> DROPPED!!
Shared data set to None!
Arc count after setting to None: 3
Final Arc count: 2
Shared data: None
*/
fn test_forget_03() {
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

    // Implement Drop for YourStruct to observe when it gets dropped
    impl Drop for YourStruct {
        fn drop(&mut self) {
            println!(
                "Dropping YourStruct with value: {} >>> DROPPED!!",
                self.value
            );
        }
    }

    // Define an enum with two members
    #[derive(Debug)]
    enum MyEnum {
        Int(i32),
        Struct(YourStruct),
    }

    // Implement Drop for MyEnum to observe when any of its variants are dropped
    impl Drop for MyEnum {
        fn drop(&mut self) {
            match self {
                MyEnum::Int(val) => {
                    println!("Dropping MyEnum::Int with value: {} >>> DROPPED!!", val);
                }
                MyEnum::Struct(s) => {
                    println!("Dropping MyEnum::Struct with value: {:?} >>> DROPPED!!", s);
                }
            }
        }
    }


    // Create a shared Arc object wrapping MyEnum (as Option) and set an initial value
    let shared_data = Arc::new(Mutex::new(Some(MyEnum::Struct(YourStruct::new(42)))));

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

pub fn test() {
    test_forget_01();
    test_forget_02();
    test_forget_03();
    tmp();
}

pub fn tmp() {
    struct FancyNum {
        num: usize
    }

    struct DropStruct {
        fancy: FancyNum
    }

    impl Drop for DropStruct {
        fn drop(&mut self) {
            // Destruct DropStruct, possibly using FancyNum
        }
    }

    {
        let drop_struct = DropStruct{fancy: FancyNum{num: 5}};
        let ref fancy_field = drop_struct.fancy; // No more errors!
        println!("Fancy: {}", fancy_field.num);
        // implicit call to `drop_struct.drop()` as drop_struct goes out of scope
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_forget() {
        test_forget_03();
    }
}