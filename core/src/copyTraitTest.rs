// This will demostration of copy trait
// Few Points to note:
// 1. Copy trait is used to copy the value of a variable to another variable
// 2. Copy trait for a type is automatically implemented if the type implements the Clone trait
// 3. Copy trait is implemented for all the primitive types
// 4. Copy trait is not implemented for the non-primitive types
// 5. Copy trait is implemented for the types that have a fixed size at compile time

use std::f32::consts::E;

use bitvec::slice;

fn copy_trait_primitive_types1() {
    // Copy trait for primitive types
    let a = 10;
    let mut b = a;
    println!("a: {}, b: {}", a, b);
    b = 20;
    println!("a: {}, b: {}", a, b);

    let c = true;
    let mut d = c;
    println!("c: {}, d: {}", c, d);
    d = false;
    println!("c: {}, d: {}", c, d);

    let e = 'A';
    let mut f = e;
    println!("e: {}, f: {}", e, f);
    f = 'B';
    println!("e: {}, f: {}", e, f);

    let g = 10.5;
    let mut h = g;
    println!("g: {}, h: {}", g, h);
    h = 20.5;
    println!("g: {}, h: {}", g, h);
}

fn copy_trait_non_primitive_types1() {
    // String
    let a = String::from("Hello");
    // let mut b = a; // This will give an error
    let mut b = a.clone();
    println!("a: {}, b: {}", a, b);
    b.push_str(" World");
    println!("a: {}, b: {}", a, b);

    // Vec
    let c = vec![1, 2, 3];
    // let mut d = c; // This will give an error
    let mut d = c.clone();
    println!("c: {:?}, d: {:?}", c, d);
    d.push(4);
    println!("c: {:?}, d: {:?}", c, d);

    // HashMap
    let mut e = std::collections::HashMap::new();
    e.insert(1, "One");
    e.insert(2, "Two");
    // let mut f = e; // This will give an error
    let mut f = e.clone();
    println!("e: {:?}, f: {:?}", e, f);
    f.insert(3, "Three");
    println!("e: {:?}, f: {:?}", e, f);

    /*
    // Reference
    let r = 10;
    let s = &r;
    // let mut t = s; // This will give an error
    let mut t = s.clone();
    println!("s: {}, t: {}", s, t);
    // *t = 20; // This will give an error
    println!("s: {}, t: {}", s, t);
    */
}

fn struct_test() {
    // Struct with no Copy trait
    struct Person {
        name: String,
        age: u8,
    }
    let g = Person {
        name: String::from("John"),
        age: 25,
    };
    let mut h = g; // moved because no copy trait
    // println!("g: {} {}, h: {} {}", g.name, g.age, h.name, h.age); // g.age -> value borrowed here after move
    h.name = String::from("Doe");
    h.age = 30;
    // println!("g: {} {}, h: {} {}", g.name, g.age, h.name, h.age); // g.age -> value borrowed here after move

    // Struct with Copy trait
    #[derive(Copy, Clone)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let i = Rectangle {
        width: 10,
        height: 20,
    };
    let mut j = i; // copied because of copy trait
    println!("i: {}x{}, j: {}x{}", i.width, i.height, j.width, j.height);
    j.width = 30;
    j.height = 40;
    println!("i: {}x{}, j: {}x{}", i.width, i.height, j.width, j.height);


    // Struct with Copy trait 
    struct Circle {
        radius: u32,
    }
    impl Copy for Circle {} // Copy trait implemented manually
    impl Clone for Circle { // Clone trait implemented manually
        fn clone(&self) -> Circle {
            *self
        }
    }
    let k = Circle {
        radius: 10,
    };
    let mut l = k; // copied because of copy trait
    println!("k: {}, l: {}", k.radius, l.radius);
    l.radius = 20;
    println!("k: {}, l: {}", k.radius, l.radius);

    // Difference between Copy and Clone trait
    #[derive(Copy, Clone)]
    struct Square {
        side: u32,
    }
    let m = Square {
        side: 10,
    };
    let n = m; // copied because of copy trait
    println!("m: {}, n: {}", m.side, n.side);
    let o = m.clone(); // cloned because of clone trait
    println!("m: {}, o: {}", m.side, o.side);
    // println!("m: {}, n: {}, o: {}", m.side, n.side, o.side); // m.side -> value borrowed here after move

}


fn enum_test() {
    // Enum with no Copy trait
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let a = Color::Red;
    let mut b = a; // moved because no copy trait
    // println!("a: {:?}, b: {:?}", a, b); // a -> value borrowed here after move
    b = Color::Green;
    // println!("a: {:?}, b: {:?}", a, b); // a -> value borrowed here after move

    #[derive(Clone)]
    #[derive(Debug)]
    enum Color2 {
        Red,
        Green,
        Blue,
    }
    let i = Color2::Red;
    // let mut j = i; // This will give an error
    let mut j = i.clone();
    println!("i: {:?}, j: {:?}", i, j);
    j = Color2::Green;
    println!("i: {:?}, j: {:?}", i, j);

    // Enum with Copy trait
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    enum Shape {
        Circle,
        Square,
        Rectangle,
    }
    let c = Shape::Circle;
    let mut d = c; // copied because of copy trait
    println!("c: {:?}, d: {:?}", c, d);
    d = Shape::Square;
    println!("c: {:?}, d: {:?}", c, d);

    // Enum with Copy trait
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let e = Direction::Up;
    let mut f = e.clone(); // copied because of copy trait
    println!("e: {:?}, f: {:?}", e, f);
    f = Direction::Down;
    println!("e: {:?}, f: {:?}", e, f);

    // impl Copy for enum manually
    #[derive(Debug)]
    enum Color3 {
        Red,
        Green,
        Blue,
    }
    impl Copy for Color3 {} // Copy trait implemented manually
    impl Clone for Color3 { // Clone trait implemented manually
        fn clone(&self) -> Color3 {
            *self
        }
    }
    let g = Color3::Red;
    let mut h = g; // copied because of copy trait
    println!("g: {:?}, h: {:?}", g, h);
    h = Color3::Green;
    println!("g: {:?}, h: {:?}", g, h);
}

fn touple_test() {
    // Tuple
    let k = (10, 20);
    // let mut l = k; // This will give an error
    let mut l = k.clone();
    println!("k: {:?}, l: {:?}", k, l);
    l = (30, 40);
    println!("k: {:?}, l: {:?}", k, l);

    // Tuple with Copy trait
    #[derive(Debug)]
    #[derive(Copy, Clone)]
    struct Point(u32, u32);
    let m = Point(10, 20);
    let mut n = m; // copied because of copy trait
    println!("m: {:?}, n: {:?}", m, n);
    n = Point(30, 40);
    println!("m: {:?}, n: {:?}", m, n);

    // Tuple with Copy trait, manually implemented
    #[derive(Debug)]
    struct Point2(u32, u32);
    impl Copy for Point2 {} // Copy trait implemented manually
    impl Clone for Point2 { // Clone trait implemented manually
        fn clone(&self) -> Point2 {
            *self
        }
    }
    let o = Point2(10, 20);
    let mut p = o; // copied because of copy trait
    println!("o: {:?}, p: {:?}", o, p);
    p = Point2(30, 40);
    println!("o: {:?}, p: {:?}", o, p);

}

fn array_test() {
     // Array
     let m = [1, 2, 3];
     // let mut n = m; // This will give an error
     let mut n = m.clone();
     println!("m: {:?}, n: {:?}", m, n);
     n[0] = 10;
     println!("m: {:?}, n: {:?}", m, n);

    // Array with Copy trait
    #[derive(Debug)]
    #[derive(Copy, Clone)]
    struct Point3([u32; 2]);
    let o = Point3([10, 20]);
    let mut p = o; // copied because of copy trait
    println!("o: {:?}, p: {:?}", o, p);
    p.0[0] = 100 as u32;
    println!("o: {:?}, p: {:?}", o, p);


    // Array with Copy trait, manually implemented
    #[derive(Debug)]
    struct Point4([u32; 2]);
    impl Copy for Point4 {} // Copy trait implemented manually
    impl Clone for Point4 { // Clone trait implemented manually
        fn clone(&self) -> Point4 {
            *self
        }
    }
    let q = Point4([10, 20]);
    let mut r = q; // copied because of copy trait
    println!("q: {:?}, r: {:?}", q, r);
    r.0[0] = 100;
    println!("q: {:?}, r: {:?}", q, r);

}

fn slice_test() {
    // slice are read only
    let mut arr = [1, 2, 3, 4, 5];
    let mut slice = &arr[0..3];
    // slice[2] = 10; // This will give an error : `slice` is a `&` reference, so the data it refers to cannot be written

    // Slice
    let o = [1, 2, 3, 4, 5];
    let p = &o[0..3];
    // let mut q = p; // This will give an error
    let mut q = p.clone();
    println!("p: {:?}, q: {:?}", p, q);
    // q[0] = 10; // This will give an error
    println!("p: {:?}, q: {:?}", p, q);

    // Slice with Copy trait
    #[derive(Debug)]
    #[derive(Copy, Clone)]
    struct Point5([u32; 5]);
    let r = Point5([1, 2, 3, 4, 5]);
    let s = &r.0[0..3];
    let mut t = s; // copied because of copy trait
    println!("s: {:?}, t: {:?}", s, t);
    // t[0] = 10; // This will give an error
    println!("s: {:?}, t: {:?}", s, t);

    // Slice with Copy trait, manually implemented
    #[derive(Debug)]
    struct Point6([u32; 5]);
    impl Copy for Point6 {} // Copy trait implemented manually
    impl Clone for Point6 { // Clone trait implemented manually
        fn clone(&self) -> Point6 {
            *self
        }
    }
    let u = Point6([1, 2, 3, 4, 5]);
    let v = &u.0[0..3];
    let mut w = v; // copied because of copy trait
    println!("v: {:?}, w: {:?}", v, w);
    // w[0] = 10; // This will give an error
    println!("v: {:?}, w: {:?}", v, w);

}
fn copy_trait_primitive_types() {
    copy_trait_primitive_types1();
}

fn copy_trait_non_primitive_types() {
    copy_trait_non_primitive_types1();
}

///
/// Test the Copy trait
/// We are testing the following:
/// - copy_trait_primitive_types
/// - copy_trait_non_primitive_types
/// - struct_test
/// - enum_test
/// - touple_test
/// - array_test
/// - slice_test
/// 
pub fn test() {
    // Copy trait for primitive types
    copy_trait_primitive_types();
    copy_trait_non_primitive_types();
    struct_test();
    enum_test();
    touple_test();
    array_test();
    slice_test();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_test() {
        slice_test();
    }

    #[test]
    fn test_array_test() {
        array_test();
    }

    #[test]
    fn test_copy_trait_primitive_types() {
        copy_trait_primitive_types();
    }

    #[test]
    fn test_copy_trait_non_primitive_types() {
        copy_trait_non_primitive_types1();
    }

    #[test]
    fn test_struct_test() {
        struct_test();
    }

    #[test]
    fn test_enum_test() {
        enum_test();
    }

    #[test]
    fn test_touple_test() {
        touple_test();
    }
}