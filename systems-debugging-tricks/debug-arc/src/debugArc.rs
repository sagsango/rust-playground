use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::Deref;
use std::thread::JoinHandle;

/*
We have on struct Data which is being used by diff threads
And we want to trace the creation, increment and decrement of the reference count and drop of the object

Only one thing the struct Data has to do is to implement the Traced trait; this is one of the ways to implement the tracing

*/
// Define a trait for tracing
pub trait Traced: Sized {
    fn on_create() {
        println!("[LOG] New object created");
    }

    fn on_increment_ref_count(count: usize) {
        println!("[LOG] Reference count incremented (strong count = {})", count);
    }

    fn on_decrement_ref_count(count: usize) {
        println!("[LOG] Reference count decremented (strong count = {})", count);
    }

    fn on_drop() {
        println!("[LOG] Object will be dropped");
    }
}

// Wrapper to add tracing functionality
pub struct TracedArc<T: Traced> {
    inner: Arc<T>,
    ref_count: Arc<AtomicUsize>, // To track the reference count manually
}

impl<T: Traced> TracedArc<T> {
    pub fn new(data: T) -> Self {
        T::on_create();
        TracedArc {
            inner: Arc::new(data),
            ref_count: Arc::new(AtomicUsize::new(1)),
        }
    }

    pub fn clone(&self) -> Self {
        let count = self.ref_count.fetch_add(1, Ordering::SeqCst) + 1;
        T::on_increment_ref_count(count);
        TracedArc {
            inner: Arc::clone(&self.inner),
            ref_count: Arc::clone(&self.ref_count),
        }
    }
}

// Deref implementation to allow transparent access to the underlying data
impl<T: Traced> Deref for TracedArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Drop implementation to log when an object is dropped
impl<T: Traced> Drop for TracedArc<T> {
    fn drop(&mut self) {
        let count = self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1;
        T::on_decrement_ref_count(count);
        if count == 0 {
            T::on_drop();
        }
    }
}

// Example struct
pub struct Data {
    pub value: i32,
}

// Implement Traced for the example struct
impl Traced for Data {}

// Main function to demonstrate usage
fn test_debugger_single_thread() {
    let data = TracedArc::new(Data { value: 42 }); // Logs object creation

    {
        let data_clone = data.clone(); // Logs reference count increment
        println!("Value from clone: {}", data_clone.value);
    } // Logs reference count decrement

    println!("Value from original: {}", data.value);
} // Logs object drop


fn test_debuuger_multithread() {
    let data = TracedArc::new(Data { value: 42 });
    let handle:JoinHandle<()>;
    {
        let data_clone = data.clone();
        handle = std::thread::spawn(move || {
            println!("Value from clone: {}", data_clone.value);

            let data_clone2 = data_clone.clone();

            println!("Value from clone2: {}", data_clone2.value);
        });
    }

    handle.join().unwrap();
    println!("Value from original: {}", data.value);
}


pub fn test() {
    test_debugger_single_thread();
    test_debuuger_multithread();
}


#[test]
fn test_debugArc_single_thread() {
    test_debugger_single_thread();
}

#[test]
fn test_debugArc_multithread() {
    test_debuuger_multithread();
}