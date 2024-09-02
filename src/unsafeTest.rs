// This file contains the diff usecase of unsafe keyword in rust
// unsafe keyword is used to bypass the borrow checker and access the memory directly
// unsafe keyword is used to define unsafe pointers
// unsafe keyword is used to define unsafe traits
// unsafe keyword is used to define unsafe structs
// unsafe keyword is used to define unsafe impl
// unsafe keyword is used to define unsafe blocks
// unsafe keyword is used to define unsafe functions
// unsafe keyword is used to define unsafe variables


fn test_unsafe_impl1() {
    // unsafe impl
    struct Foo {
        x: i32,
    }
    impl Foo {
        fn new(x: i32) -> Self {
            Foo { x }
        }
        unsafe fn dangerous(&self) {
            println!("x: {}", self.x);
        }
    }
    let f = Foo::new(10);
    unsafe {
        f.dangerous();
    }
}

fn test_unsafe_struct1() {
    // there is no unsafe struct
    /*
        to declare the existence of contracts the compiler can't check (unsafe fn and unsafe trait),
        and to declare that a programmer has checked that these contracts have been upheld (unsafe {} and unsafe impl, but also unsafe fn -- see below).
    */
    // unsafe struct Foo { // there is no unsafe struct
    struct Foo <'a>{ 
        x: i32,
        y: &'a i32,

    }

    let mut z: i32 = 10;
    // print the address of z
    println!("After z allocation, z: {:p}", &z);
    let f = Foo { x: 10 , y : &z};
    // thread which will print z, we will wait for this thread to finish
    std::thread::spawn(move || { /* XXX: This thread will have own copy of the z, becase Rust disallow multiple owner of lifetime */
        // z has been moved to the thread
        println!("In child thread, z: {:p}", &z);
        println!("z: {}", z);
        z = 20;
        println!("z: {}", z);
    }).join().unwrap();
    println!("In parent thread, z: {:p}", &z);
    println!("x: {}", *f.y); // f.y is still safe because it has lifetime checking


    {
        struct Foo <'a>{ 
            x: i32,
            y: &'a i32,
    
        }

        let z: i32 = 10;
        // print the address of z
        println!("After z allocation, z: {:p}", &z);
        let f = Foo { x: 10 , y : &z};
        // thread which will print z, we will wait for this thread to finish
        std::thread::spawn(move || { /* XXX: This thread will have own copy of the z, becase Rust disallow multiple owner of lifetime */
            // z has been moved to the thread
            println!("In child thread, z: {:p}", &z);
            println!("z: {}", z);
        }).join().unwrap();
        println!("In parent thread, z: {:p}", &z);
        println!("x: {}", *f.y); // f.y is still safe because it has lifetime checking
   }

   /*{
        // Why rust is memory safe : https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
        // error[E0597]: `z` does not live long enough
        // `z` does not live long enough
        // borrowed value does not live long enoughrustcClick for full compiler diagnostic
        // unsafeTest.rs(98, 4): `z` dropped here while still borrowed
        // unsafeTest.rs(89, 13): binding `z` declared here
        // unsafeTest.rs(93, 9): argument requires that `z` is borrowed for `'static`
        // Why : Because we dont know what is the lifetime of the child thread, so we can't pass the reference of z to the child thread

        #[derive(Debug)]
        struct Foo <'a>{ 
            x: i32,
            y: &'a mut i32,
    
        }
        
        let mut z: i32 = 10;
        // print the address of z
        let f = Foo { x: 10 , y : &mut z};
        // thread which will print z, we will wait for this thread to finish
        std::thread::spawn(move || { /* XXX: This thread will have own copy of the z, becase Rust disallow multiple owner of lifetime */
            // z has been moved to the thread
            println!("In child thread, f: {:?}",f);
        }).join().unwrap();
        println!("In parent thread, f: {:?}", &f);
   }*/


}   


fn test_unsafe_trait1() {
    // unsafe trait
    unsafe trait Foo {
        fn foo(&self);
    }
    struct Bar;
    unsafe impl Foo for Bar { // unsafe impl
        fn foo(&self) {
            println!("Foo");
        }
    }
    let b = Bar;
    b.foo();
}

fn test_unsafe_pointer1() {
    // unsafe pointer
    let x = 10;
    let ptr_x = &x as *const i32;
    unsafe {
        println!("x: {}", *ptr_x);
    }
}

fn test_unsafe_variable1() {
    // unsafe variable
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    unsafe {
        *ptr_x += 10;
        println!("x: {}", *ptr_x);
    }
}

fn test_unsafe_function1() {
    // unsafe function
    unsafe fn dangerous() {
        println!("This is an unsafe function");
    }
    unsafe { 
        dangerous(); 
    }
}

fn test_unsafe_block1() {
     // unsafe block
     unsafe {
        println!("This is an unsafe block");
    }
}
fn test_unsafe_impl() {
    test_unsafe_impl1();
}

fn test_unsafe_struct() {
    test_unsafe_struct1();
}

fn test_unsafe_trait() {
    test_unsafe_trait1();
}
fn test_unsafe_pointer() {
    test_unsafe_pointer1();
}

fn test_unsafe_variable() {
    test_unsafe_variable1();
}   

fn test_unsafe_function() {
   test_unsafe_function1();
}

fn test_unsafe_block() {
   test_unsafe_block1();
}

pub fn test() {
    test_unsafe_block();
    test_unsafe_function();
    test_unsafe_variable();
    test_unsafe_pointer();
    test_unsafe_trait();
    test_unsafe_struct();
    test_unsafe_impl();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        test();
    }
    
    #[test]
    fn test_test_unsafe_block() {
        test_unsafe_block();
    }

    #[test]
    fn test_test_unsafe_function() {
        test_unsafe_function();
    }

    #[test]
    fn test_test_unsafe_variable() {
        test_unsafe_variable();
    }

    #[test]
    fn test_test_unsafe_pointer() {
        test_unsafe_pointer();
    }

    #[test]
    fn test_test_unsafe_trait() {
        test_unsafe_trait();
    }

    #[test]
    fn test_test_unsafe_struct() {
        test_unsafe_struct();
    }

    #[test]
    fn test_test_unsafe_impl() {
        test_unsafe_impl();
    }
}
