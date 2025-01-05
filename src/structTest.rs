/*  
NOTES on Copy and Clone:
1. When Struct already implementes Drop, which disallows Copy. (Because Distructure will be called no matter what)
2. Struct which have all the members which implement Copy, then we can implement Copy for the struct.
3. Struct which have all the members which implement Clone, then we can implement Clone for the struct.
4. If we implement Copy, then we should not implement Clone. (Because Copy is shallow copy, and Clone is deep copy and disctructor will be called)
5. Copy is alread by rust with bitwise copy, so we do not need to write the copy method.
6. Clone we have to implement ourself, means we have to write the clone method.



NOTES on Drop:
1. Drop is called when the object goes out of scope (and arc = 0)
2. Drop of the struct will call the drop of all the members of the struct.
3. Drop of individual object will be called in the reverse order of the declaration of the object.
4. we can use mem::drop to drop the referance before it goes out of scope. (NOTE : it will not call the drop of the object, it will just drop the referance)
*/

use std::mem;
/* How to use struct with variable size data */
struct MyStruct<'lifetime_1, 'lifetime_2> {
    data: Vec<u8>,
    name: String, // XXX: String have variable size, but its is allocaed on heap
    age: u32,
    /* data_1: str, // XXX: str size in not known at compile time so it is not allowed
                            Also str can not be passed to the functions, because size is not known at compile time
                            So we always pass &str */

    data_1: &'lifetime_1 str, // XXX: references have fixed size, but need lifetime parameter
    data_2: &'lifetime_2 str,
}

impl <'lifetime_1, 'lifetime_2> MyStruct<'lifetime_1, 'lifetime_2> {
    fn new(data: Vec<u8>, name: String, age: u32, data_1: &'lifetime_1 str, data_2: &'lifetime_2 str) -> Self {
        MyStruct {
            data,
            name,
            age,
            data_1,
            data_2,
        }
    }
}

///
/// This function is used to test the struct with variable size data
/// struct in rust is a fixed size data structure
/// But we can use references to variable size data
/// they can have member functions
/// We can add padding to the struct to make it fixed size
/// In case of references, we need to add lifetime parameter
/// 
/// 
fn lifetime_test() {
    let data = vec![1, 2, 3, 4, 5];
    let name = String::from("Joe");
    let age = 32;
    let data_1 = "Hello, World!";
    let data_2 = "Hello, Rust!";

    let my_struct = MyStruct {
        data,
        name,
        age,
        data_1,
        data_2,
    };

    println!("Size of MyStruct: {}", mem::size_of_val(&my_struct));
    println!("Data: {:?}", my_struct.data);
    println!("Name: {}", my_struct.name);
    println!("Age: {}", my_struct.age);
    println!("Data 1: {}", my_struct.data_1);
    println!("Data 2: {}", my_struct.data_2);

    let data = vec![1, 2, 3, 4, 5];
    let name = String::from("Joe");
    let age = 32;
    let data_1 = "Hello, World!";
    let data_2 = "Hello, Rust!";
    let my_struct = MyStruct::new(data, name, age, data_1, data_2);
    println!("Size of MyStruct: {}", mem::size_of_val(&my_struct));
    println!("Data: {:?}", my_struct.data);
    println!("Name: {}", my_struct.name);
    println!("Age: {}", my_struct.age);
    println!("Data 1: {}", my_struct.data_1);
    println!("Data 2: {}", my_struct.data_2);
}


fn struct_with_closure() {
    let mut my_struct = MyStruct::new(vec![1, 2, 3, 4, 5], String::from("Joe"), 32, "Hello, World!", "Hello, Rust!");
    let closure = || {
        println!("Data: {:?}", my_struct.data);
        println!("Name: {}", my_struct.name);
        println!("Age: {}", my_struct.age);
        println!("Data 1: {}", my_struct.data_1);
        println!("Data 2: {}", my_struct.data_2);
    };

    closure();
}


fn struct_with_clone_debug_drop() {
    struct MyStruct {
        data: Vec<u8>,
        name: String,
        age: u32,
    }
    impl Clone for MyStruct {
        fn clone(&self) -> Self {
            MyStruct {
                data: self.data.clone(),
                name: self.name.clone(),
                age: self.age,
            }
        }
    }

    // impl Copy for MyStruct {} 
    /* Copy can not be implemented as copy can bot be done with shallow copy (here we have heap allocated data)*/

    impl std::fmt::Debug for MyStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyStruct {{ data: {:?}, name: {}, age: {} }}", self.data, self.name, self.age)
        }
    }

    impl Drop for MyStruct {
        fn drop(&mut self) {
            println!("Dropping MyStruct with data: {:?}", self.data);
        }
    }

    let my_struct = MyStruct {
        data: vec![1, 2, 3, 4, 5],
        name: String::from("Joe"),
        age: 32,
    };

    let my_struct_clone = my_struct.clone();
    println!("my_struct: {:?}", my_struct);

    let my_struct_moved = my_struct; // moved
    println!("my_struct_moved: {:?}", my_struct_moved);

    drop(my_struct_moved);
    println!("my_struct_clone: {:?}", my_struct_clone);


    {
        // shallow copy for struct 
        struct MyStruct {
        int : i32,
            float: f32,
        }

        impl Clone for MyStruct {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Copy for MyStruct {}

        impl std::fmt::Debug for MyStruct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "MyStruct {{ int: {}, float: {} }}", self.int, self.float)
            }
        }

        // Copy can not be implemented for struct with destructor
        /*impl Drop for MyStruct {
            fn drop(&mut self) {
                println!("Dropping MyStruct with int: {}, float: {}", self.int, self.float);
            }
        }*/

        let my_struct = MyStruct {
            int: 42,
            float: 3.14,
        };

        let my_struct_clone = my_struct.clone();

        println!("my_struct: {:?}", my_struct);
        println!("my_struct_clone: {:?}", my_struct_clone);
        // drop(my_struct); 
        // calls to `std::mem::drop` with a value that implements `Copy` does nothing           (Vecause it must have been copied, and do not know how long it will be used)
        // `#[warn(dropping_copy_types)]` on by defaultrustcClick for full compiler diagnostic
        println!("my_struct_clone: {:?}", my_struct_clone);

        let copy_struct = my_struct;
        println!("copy_struct: {:?}", copy_struct);

        /*
            Always use Copy or Clone but not both;
            If you implement Copy, then you should not implement Clone
            If you implement Clone, then you should not implement Copy
        */
    }


    {
        // shallow copy for struct 
        struct MyStruct {
            int : i32,
            float: f32,
        }

        impl Clone for MyStruct {
            fn clone(&self) -> Self {
                MyStruct {
                    int: self.int,
                    float: self.float,
                }
            }
        }

        impl std::fmt::Debug for MyStruct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "MyStruct {{ int: {}, float: {} }}", self.int, self.float)
            }
        }

        let my_struct = MyStruct {
            int: 42,
            float: 3.14,
        };

        let my_struct_clone = my_struct.clone();

        println!("my_struct: {:?}", my_struct);
        println!("my_struct_clone: {:?}", my_struct_clone);

        drop(my_struct_clone);
    }

}

fn memDrop_vs_drop() {
    struct MyStruct {
        data: Vec<u8>,
        name: String,
        age: u32,
    }

    impl Drop for MyStruct {
        fn drop(&mut self) {
            println!("Dropping MyStruct with data: {:?}", self.data);
        }
    }

    let my_struct = MyStruct {
        data: vec![1, 2, 3, 4, 5],
        name: String::from("Joe"),
        age: 32,
    };

    drop(my_struct); // calls drop
    // mem::drop(my_struct);


    #[derive(Debug)]
    struct MyStruct2 {
        data: Vec<u8>,
        name: String,
        age: u32,
    }

    let my_struct2 = MyStruct2 {
        data: vec![1, 2, 3, 4, 5],
        name: String::from("Joe"),
        age: 32,
    };

    impl Drop for MyStruct2 {
        fn drop(&mut self) {
            println!("Dropping MyStruct2 with data: {:?}", self.data);
        }
    }

    println!(" my_struct2 will be droped by the standard Rust drop mechanism");
    println!(" my_struct has already been droped by the mem::drop mechanism");
}



use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Node {
    id: i32,
    next: Option<Arc<Mutex<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node with id: {}", self.id);
    }
}

fn test0() {
    println!("Starting test0:");
    let a = Arc::new(Mutex::new(Node { id: 0, next: None }));
    {
        let b = Arc::new(Mutex::new(Node { id: 1, next: None }));
        let mut node_a = a.lock().unwrap();
        node_a.next = Some(Arc::clone(&b)); // Use Weak reference
        
        println!("Strong count after creation: {}", Arc::strong_count(&b));
    }
    // Arc of a = 1
    // Arc of b = 1
}

fn test1() {
    println!("Starting test1:");
    // Create an Arc<Node> with a Mutex for interior mutability
    let a = Arc::new(Mutex::new(Node { id: 0, next: None }));

    // Lock the mutex and set `next` to create a cycle
    // let mut node_a = a.lock().unwrap();
    // node_a.next = Some(Arc::clone(&a)); // No compiler error
}

fn test2() {
    println!("Starting test2:");
    // Create an Arc<Node> with a Mutex for interior mutability
    let a = Arc::new(Mutex::new(Node { id: 0, next: None }));

    {
        // Lock the mutex and set `next` to create a cycle
        let mut node = a.lock().unwrap();
        node.next = Some(Arc::clone(&a)); // No compiler error
    }
    println!("Strong count after creation: {}", Arc::strong_count(&a));
}

fn test3() {
    println!("Starting test3:");
    let a = Arc::new(Mutex::new(Node { id: 0, next: None }));
    {
        let b = Arc::new(Mutex::new(Node { id: 1, next: Some(Arc::clone(&a)) }));
        let mut node_a = a.lock().unwrap();
        node_a.next = Some(Arc::clone(&b)); // Use Weak reference
        
        let _b = b.clone(); // NOTE: refcount will drop but drop() will be only called when refcount = 0
        
        //let mut node_b = b.lock().unwrap();
        //node_b.next = Some(Arc::clone(&a)); // Use Weak reference
        
        println!("Strong count after creation: {}", Arc::strong_count(&a));
        println!("Strong count after creation: {}", Arc::strong_count(&b));
    }
    println!("Strong count after creation: {}", Arc::strong_count(&a));
    //println!("Strong count after creation: {}", Arc::strong_count(a.lock().unwrap().next.unwrap().lock().unwrap()));
    //println!("Strong count after creation: {}", Arc::strong_count(&a.lock().unwrap().next.some().lock().unwrap()));
    // Arc of a = 2
    // Arc of b = 1
    
    
    /*
        "WHY-MEMORY-LEAK"
        Now a goes out of scope:
            refcount a should be decremented from 2 to 1
            But droped() will not be called for a because a refcount is still 1
            Because calling the drop() for b is responisibily of the drop of a, so drop of b will not be called too.
            
            So This is a memory leak
    */
}

fn memory_leacks_test() {
    test0();
    test1();
    test2();
    test3();
    println!("Main Done!");
}


use std::rc::Rc;

struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
    }
}

fn memdrop_test0() {
    println!("Test0 starts:");
    
    let r1 = Resource {
        name: String::from("Resource1"),
    };
    let r2 = Resource {
        name: String::from("Resource2"),
    };

    println!("Before calling drop()");

    // Explicitly drop r1
    std::mem::drop(r1);
    println!("After dropping r1");
    
    
    
}

fn memdrop_test1() {
    println!("Starting test1:");

    // Create a reference-counted Resource
    let r1 = Rc::new(Resource {
        name: String::from("SharedResource"),
    });

    // Print the initial strong count
    println!("Initial strong count: {}", Rc::strong_count(&r1));

    {
        let r2 = Rc::clone(&r1); // Clone increases the strong count
        println!("Strong count after cloning to r2: {}", Rc::strong_count(&r1));
        
        std::mem::drop(r2);
        
        println!("Strong count after mem::drop() to r2: {}", Rc::strong_count(&r1));
        
    } // r2 goes out of scope, decreasing the strong count

    println!(
        "Strong count after r2 goes out of scope (but drop happened on mem::drop()): {}",
        Rc::strong_count(&r1)
    );

    // r1 will be dropped at the end of this function
}


fn memdrop_tests() {
    memdrop_test0();
    memdrop_test1();
}

pub fn test() {
    lifetime_test();
    struct_with_closure();
    struct_with_clone_debug_drop();
    memory_leacks_test();
    memdrop_tests();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        test();
    }

    #[test]
    fn _test_lifetime() {
        lifetime_test();
    }

    #[test]
    fn _test_struct_with_closure() {
        struct_with_closure();
    }

    #[test]
    fn _test_struct_with_clone_debug_drop() {
        struct_with_clone_debug_drop();
    }

    #[test]
    fn _test_memDrop_vs_drop() {
        memDrop_vs_drop();
    }

    #[test]
    fn _test_memory_leacks_test() {
        memory_leacks_test();
    }
}



