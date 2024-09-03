
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
pub fn test() {
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



