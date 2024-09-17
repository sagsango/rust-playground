// std::mem module

use std::mem;

/*
    mem module have there following functolity:
    - size_of
    - forget
    - swap
    - transmute
    - align_of
    - drop
    - replace
    - zeroed
    - uninitialized
    - discriminant
    - needs_drop
    - type_id
    - size_of_val
    - min_align_of
    - align_of_val
    - drop_in_place
    - write_bytes
    - copy
    - replace_with
    - uninitialized_array
    - zeroed_array
    - ManuallyDrop
    - MaybeUninit
    - MaybeUninitSlice
    - MaybeUninitArray
    - MaybeUninitRef
    - MaybeUninitBox
    - etc.
*/

fn test_size_of() {
    println!("Size of i32: {}", mem::size_of::<i32>());
    println!("Size of f64: {}", mem::size_of::<f64>());
    println!("Size of char: {}", mem::size_of::<char>());
    println!("Size of bool: {}", mem::size_of::<bool>());

    println!("Size of [i32; 0]: {}", mem::size_of::<[i32; 0]>());
    println!("Size of [i32; 1]: {}", mem::size_of::<[i32; 1]>());
    
    // size of a struct
    struct MyStruct {
        a: i32,
        b: f64,
        c: char,
        d: bool,
    }
    println!("Size of MyStruct: {}", mem::size_of::<MyStruct>());

    println!("Size of String: {}", mem::size_of::<String>());
    println!("Size of &str: {}", mem::size_of::<&str>());
    println!("Size of Box<i32>: {}", mem::size_of::<Box<i32>>());
    println!("Size of Option<i32>: {}", mem::size_of::<Option<i32>>());
    println!("Size of Result<i32, i32>: {}", mem::size_of::<Result<i32, i32>>());
    println!("Size of Vec<i32>: {}", mem::size_of::<Vec<i32>>());
    println!("Size of Vec<i32>: {}", mem::size_of::<Vec<i32>>());
}


fn test_forget() {
    let x = Box::new(42);
    println!("x: {}", x);
    mem::forget(x); // This will not deallocate the memory
    // println!("x: {}", x); // This will give an error

    // on a struct 
    #[derive(Debug)]
    struct MyStruct {
        a: i32,
        b: f64,
        c: char,
        d: bool,
    }
    impl Drop for MyStruct {
        fn drop(&mut self) {
            println!("Dropping MyStruct with a: {}, b: {}, c: {}, d: {}", self.a, self.b, self.c, self.d);
        }
    }
    
    // box of struct
    let x = Box::new(MyStruct { a: 42, b: 3.14, c: 'x', d: true });
    println!("x: {:?}", x);

    // first drop the box
    drop(x);

    // now, the struct will be dropped
    // you will see drop is called
    // println!("x: {:?}", x); // This will give an error


    // lets forget the box
    let x = Box::new(MyStruct { a: 42, b: 3.14, c: 'x', d: true });
    println!("x: {:?}", x);
    mem::forget(x);
    // now, the struct will not be dropped
    // and you will get a memory leak; drop will not be called
    
}


fn test_swap() {
    let mut x = 42;
    let mut y = 43;
    println!("x: {}, y: {}", x, y);
    mem::swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y);

    let mut x = Box::new(42);
    let mut y = Box::new(43);
    println!("x: {}, y: {}", x, y);
    mem::swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y);

    // on a struct; with member vector
    #[derive(Debug)]
    struct MyStruct {
        a: i32,
        b: f64,
        c: char,
        d: bool,
        e: Vec<i32>,
    }

    let mut x = Box::new(MyStruct { a: 42, b: 3.14, c: 'x', d: true, e: vec![1, 2, 3] });

    let mut y = Box::new(MyStruct { a: 43, b: 3.15, c: 'y', d: false, e: vec![4, 5, 6] });

    println!("x: {:?}", x);
    println!("y: {:?}", y);

    mem::swap(&mut x.e, &mut y.e);

    println!("x: {:?}", x);
    println!("y: {:?}", y);
}

pub fn test() {
    test_size_of();
    test_forget();
    test_swap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem() {
        test();
    }

    #[test]
    fn _test_size_of() {
        test_size_of();
    }

    #[test]
    fn _test_forget() {
        test_forget();
    }

    #[test]
    fn _test_swap() {
        test_swap();
    }
}