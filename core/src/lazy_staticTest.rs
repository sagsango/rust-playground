/*
In Rust, `lazy_static` is a macro that allows you to define static variables that are lazily initialized.
This means that the variables are not initialized until they are first accessed, and after that, they are cached and reused.
This is useful when you need to initialize global variables that require complex initialization or cannot be evaluated at compile time.

### Why Use `lazy_static`?

Rust’s `const` and `static` allow defining global variables, but they must be initialized with constant expressions
that can be evaluated at compile time. If you need more flexibility, such as running code to initialize a global variable, you use `lazy_static`.

`lazy_static` ensures that the initialization happens only once and is safe in a multithreaded context.

*/

mod test01 {
    use lazy_static;
    use std::sync::Mutex;
    use std::collections::HashMap;
    lazy_static::lazy_static! {
        static ref CONFIG: HashMap<String, String> = {
            let mut m = HashMap::new();
            m.insert("key1".to_string(), "value1".to_string());
            m.insert("key2".to_string(), "value2".to_string());
            m
        };

        static ref COUNTER: Mutex<u32> = Mutex::new(0); // Thread-safe mutable static variable
    }

    pub fn test() {
        // Access the lazy static HashMap
        println!("Config for key1: {}", CONFIG.get("key1").unwrap());

        // Modify the lazy static counter in a thread-safe manner
        {
            let mut count = COUNTER.lock().unwrap();
            *count += 1;
            println!("Counter: {}", *count);
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_lazy_static() {
            test();
        }
    }
}

mod test02 {
    use lazy_static;
    use std::sync::Mutex;
    use std::sync::Arc;
    use std::thread;

    lazy_static::lazy_static! {
        static ref GLOBAL_COUNTER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    }

    fn increment_counter() {
        let mut counter = GLOBAL_COUNTER.lock().unwrap();
        *counter += 1;
    }

    pub fn test() {
        let handles: Vec<_> = (0..10).map(|_| {
            thread::spawn(|| {
                increment_counter();
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Global counter: {}", *GLOBAL_COUNTER.lock().unwrap());
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_lazy_static() {
            test();
        }
    }
}


pub fn test() {
    test01::test();
    test02::test();
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_static() {
        test();
    }
}
