// To test generic





// Dont warn about unused variables
fn generaic_struct_01() {
    // Generics
    // https://doc.rust-lang.org/book/ch10-01-syntax.html

    // Generic struct
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // Generic struct implementation
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Diff ways of creating generaic objects
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    let _char = Point { x: 'a', y: 'b' };

    // explicit type
    let _integer: Point<i32> = Point { x: 5, y: 10 };
    let _float: Point<f32> = Point { x: 1.0, y: 4.0 };
    let _char: Point<char> = Point { x: 'a', y: 'b' };

    print!("Generaic struct Point: {:?}", _integer);



    // Generics with multiple types
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    // Generics with multiple types implementation
    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    print!("Generaic struct Point2: {:?}", p3);

}



fn generic_functions() {
    // Generics
    // https://doc.rust-lang.org/book/ch10-01-syntax.html

    // Generic function

    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
    
        for item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }
    

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}


fn generic_enums() {
    // Generics
    // https://doc.rust-lang.org/book/ch10-01-syntax.html

    // Generic enum
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }

    let integer = Option::Some(5);
    let float = Option::Some(5.0);
    let char = Option::Some('a');
    let none: Option<i32> = Option::None;

    print!("Generaic enum Option: {:?}", integer);
    print!("Generaic enum Option: {:?}", float);
    print!("Generaic enum Option: {:?}", char);
    print!("Generaic enum Option: {:?}", none);
}


fn generic_traits() {
    // Generics
    // https://doc.rust-lang.org/book/ch10-01-syntax.html

    // Generic trait
    trait Summary {
        fn summarize(&self) -> String;
    }

    // Generic struct
    #[derive(Debug)]
    struct NewsArticle {
        headline: String,
        location: String,
    }

    // Generic struct implementation
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, {}", self.headline, self.location)
        }
    }

    // objects
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
    };

    print!("Generaic struct NewsArticle: {:?}", article);

    // Generic struct
    #[derive(Debug)]
    struct Tweet {
        username: String,
        content: String,
    }

    // Generic struct implementation
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }


   // objects
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    print!("Generaic struct Tweet: {:?}", tweet);

}


// S<T> where T: R - A Trait-Bounded Generic Type
/*
This defines a generic struct or function where T must implement the trait R.
This introduces a trait bound, which ensures that T satisfies certain requirements,
such as having specific methods or behaviors.
*/
fn generic_trait_bounds_01() {
    trait Displayable {
        fn display(&self);
    }
    
    struct S<T>
    where
        T: Displayable, // Trait bound; so `T` must implement `Displayable`
    {
        value: T,
    }
    
    impl Displayable for i32 {
        fn display(&self) {
            println!("Displaying: {}", self);
        }
    }
    
   
    let instance = S { value: 42 }; // `T` is i32, which implements Displayable
    instance.value.display();      // Can call methods from Displayable
   

    // S<T: R>	Short hand bound, almost same as above, shorter to write.
    trait Displayable2 {
        fn display(&self);
    }

    struct S2<T: Displayable2> {
        value: T,
    }

    impl Displayable2 for i64 {
        fn display(&self) {
            println!("Displaying: {}", self);
        }
    }

    let instance = S2 { value: 42i64 }; // `T` is i32, which implements Displayable
    instance.value.display();      // Can call methods from Displayable
}


pub fn test() {
    generaic_struct_01();
    generic_functions();
    generic_enums();
    generic_traits();
    generic_trait_bounds_01();

}   


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generaic_struct_01() {
        generaic_struct_01();
    }

    #[test]
    fn test_generic_functions() {
        generic_functions();
    }

    #[test]
    fn test_generic_enums() {
        generic_enums();
    }

    #[test]
    fn test_generic_traits() {
        generic_traits();
    }

    #[test]
    fn test_generic_trait_bounds_01() {
        generic_trait_bounds_01();
    }

    #[test]
    fn test_all() {
        test_generaic_struct_01();
        test_generic_functions();
        test_generic_enums();
        test_generic_traits();
        test_generic_trait_bounds_01();
    }

}