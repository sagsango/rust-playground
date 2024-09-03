
// This is a static variable and functions tests
// This file contains the code for the static variable and functions test
static mut COUNTER: u32 = 0;
pub fn increment() {
    unsafe {
        COUNTER += 1;
    }
}
pub fn decrement() {
    unsafe {
        COUNTER -= 1;
    }
}
pub fn get_counter() -> u32 {
    unsafe {
        COUNTER
    }
}


// static struct test
struct Config {
    debug: bool,
    version: &'static str,
}

static CONFIG: Config = Config {
    debug: true,
    version: "1.0.0",
};


// struct with static function
struct Person {
    name: &'static str,
    age: u8,
}

impl Person {
    // static function
    fn new(name: &'static str, age: u8) -> Self {
        Person {
            name,
            age,
        }
    }
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Static function to create a new Rectangle
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Static function to return a square with given size
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Method that requires an instance of Rectangle
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

///
/// Test static
/// Static variables are similar to global variables in C
/// Static variables are shared between threads
/// Static variables are mutable
/// Static variables are unsafe
/// Static variables are not thread safe
/// Static variables are not garbage collected
/// Static variables are not dropped
/// Static variables are not initialized
/// 
pub fn test() {
    increment();
    increment();
    increment();
    decrement();
    println!("Counter: {}", get_counter());
    println!("Debug: {}, Version: {}", CONFIG.debug, CONFIG.version);
    
    let person = "Joe";
    let person = Person::new(person, 32);
    person.print();



    // Call the static function to create a new Rectangle
    let rect1 = Rectangle::new(30, 50);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // Call the static function to create a square
    let square = Rectangle::square(20);
    println!("The area of the square is {} square pixels.", square.area());
}