// Write a program where 2 there will use lock
// And lock is a member of the struct

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::backtrace::Backtrace;

fn simple_lock_test() {
    // A simple structure
    pub struct Data {
        pub value: i32,
    }

    // Type alias for shared and synchronized access to Data
    pub type DataPtr = Arc<Mutex<Data>>;

    fn test() {
        // Create a shared instance of Data
        let data_ptr: DataPtr = Arc::new(Mutex::new(Data { value: 42 }));

        // Regular lock without stack trace
        {
            let mut data = data_ptr.lock().expect("Failed to acquire lock");
            data.value += 1;
            println!("Data value after regular lock: {}", data.value);
        }

        /*  XXX: NOt a good way to do this
        // Conditional tracing: Print stack trace before locking
        let enable_tracing = true; // Toggle this to enable or disable tracing
        if enable_tracing {
            let backtrace = Backtrace::force_capture();
            println!("[DEBUG] Locking DataPtr with trace. Stack trace:\n{backtrace}");
        }*/

        // Lock again and modify the data
        {
            let mut data = data_ptr.lock().expect("Failed to acquire lock");
            data.value += 1;
            println!("Data value after traced lock: {}", data.value);
        }

        // Final lock without any tracing
        let data = data_ptr.lock().expect("Failed to acquire lock");
        println!("Final data value: {}", data.value);
    }

    test();
}

fn simple_lock_with_trace_test() {
    // A simple structure
    pub struct Data {
        pub value: i32,
    }

    // Type alias for shared and synchronized access to Data
    pub type DataPtr = Arc<Mutex<Data>>;

    // Trait for locking with tracing
    pub trait TracedLock {
        fn lock_traced(&self) -> MutexGuard<Data>;
    }

    // Implementation of TracedLock for DataPtr
    impl TracedLock for DataPtr {
        fn lock_traced(&self) -> MutexGuard<Data> {
            // Capture and print the stack trace
            let backtrace = Backtrace::force_capture();
            println!("[DEBUG] Locking DataPtr. Stack trace:\n{backtrace}");

            // Perform the actual lock by dereferencing `self`
            self.as_ref().lock().expect("Failed to acquire lock")
        }
    }

    fn test() {
        // Create a shared instance of Data
        let data_ptr: DataPtr = Arc::new(Mutex::new(Data { value: 42 }));

        // Lock the data pointer with tracing
        {
            let mut data = data_ptr.lock_traced();
            data.value += 1; // Modify the value
            println!("Data value after increment: {}", data.value);
        }

        // Lock again to demonstrate reuse
        let data = data_ptr.lock_traced();
        println!("Data value after second lock: {}", data.value);
    }

    test();
}

pub fn test() {
    simple_lock_test();
    simple_lock_with_trace_test();
}  

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _test_debugWhoTookTheLock() {
        test();
    }

    #[test]
    fn _test_simple_lock_test() {
        simple_lock_test();
    }


    #[test]
    fn _test_simple_lock_with_trace_test() {
        simple_lock_with_trace_test();
    }
}