
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
        // calls to `std::mem::drop` with a value that implements `Copy` does nothing 
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

pub fn test() {
    lifetime_test();
    struct_with_closure();
    struct_with_clone_debug_drop();
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

}



