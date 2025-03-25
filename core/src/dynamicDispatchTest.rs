

// Passing Trait Objects to Functions
fn pass_trait_object_to_function() {
    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    struct Square;

    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing a Circle");
        }
    }

    impl Drawable for Square {
        fn draw(&self) {
            println!("Drawing a Square");
        }
    }

    fn render(object: &dyn Drawable) {
        object.draw();
    }

    let circle = Circle;
    let square = Square;

    render(&circle); // Pass any type implementing Drawable
    render(&square);
}

// Storing Heterogeneous Types in Collections
fn heterogeneous_types_in_collection() {
    trait Animal {
        fn speak(&self);
    }
    
    struct Dog;
    struct Cat;
    
    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
    
    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }
    
    
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals {
        animal.speak(); // Dynamic dispatch happens here
    }
}

// Trait Objects in Structs
fn trait_object_in_strcut() {
    trait Logger {
        fn log(&self, message: &str);
    }
    
    struct ConsoleLogger;
    impl Logger for ConsoleLogger {
        fn log(&self, message: &str) {
            println!("Console: {}", message);
        }
    }
    
    struct FileLogger;
    impl Logger for FileLogger {
        fn log(&self, message: &str) {
            println!("File: {}", message);
        }
    }
    
    struct App {
        logger: Box<dyn Logger>,
    }
    
    impl App {
        fn new(logger: Box<dyn Logger>) -> Self {
            App { logger }
        }
    
        fn run(&self) {
            self.logger.log("App is running!");
        }
    }
    
    
    let console_logger = Box::new(ConsoleLogger);
    let app = App::new(console_logger);
    app.run();
}
    

pub fn test() {
    pass_trait_object_to_function();
    heterogeneous_types_in_collection();
    trait_object_in_strcut();
}   


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_dispatch() {
        test();
    }

    #[test]
    fn test_pass_trait_object_to_function() {
        pass_trait_object_to_function();
    }

    #[test]
    fn test_heterogeneous_types_in_collection() {
        heterogeneous_types_in_collection();
    }

    #[test]
    fn test_trait_object_in_strcut() {
        trait_object_in_strcut();
    }
    
}
    