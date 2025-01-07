use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::Deref;

// Define a trait for tracing
pub trait Traced: Sized {
    fn name() -> &'static str; // Method to return the structure name

    fn on_create() {
        println!("[LOG] New object of type `{}` created", Self::name());
    }

    fn on_increment_ref_count(count: usize) {
        println!(
            "[LOG] Reference count incremented for `{}` (strong count = {})",
            Self::name(),
            count
        );
    }

    fn on_decrement_ref_count(count: usize) {
        println!(
            "[LOG] Reference count decremented for `{}` (strong count = {})",
            Self::name(),
            count
        );
    }

    fn on_drop() {
        println!("[LOG] Object of type `{}` will be dropped", Self::name());
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

// Example struct 1
pub struct Data1 {
    pub value: i32,
}

// Example struct 2
pub struct Data2 {
    pub text: String,
}

// Implement Traced for Data1
impl Traced for Data1 {
    fn name() -> &'static str {
        "Data1"
    }
}

// Implement Traced for Data2
impl Traced for Data2 {
    fn name() -> &'static str {
        "Data2"
    }
}

// Main function to demonstrate usage
fn debug_signle_thread() {
    let data1 = TracedArc::new(Data1 { value: 42 }); // Logs object creation
    let data2 = TracedArc::new(Data2 {
        text: "Hello".to_string(),
    }); // Logs object creation

    {
        let data1_clone = data1.clone(); // Logs reference count increment
        println!("Value from Data1 clone: {}", data1_clone.value);
    } // Logs reference count decrement

    println!("Value from Data1 original: {}", data1.value);

    {
        let data2_clone = data2.clone(); // Logs reference count increment
        println!("Text from Data2 clone: {}", data2_clone.text);
    } // Logs reference count decrement
} // Logs object drop for both Data1 and Data2



fn debug_multi_thread() {
    let data1 = TracedArc::new(Data1 { value: 42 });
    let data2 = TracedArc::new(Data2 {
        text: "Hello".to_string(),
    });

    let handle: std::thread::JoinHandle<()>;
    {
        let data1_clone = data1.clone();
        let data2_clone = data2.clone();
        handle = std::thread::spawn(move || {
            println!("Value from Data1 clone: {}", data1_clone.value);
            println!("Text from Data2 clone: {}", data2_clone.text);

            let data1_clone2 = data1_clone.clone();
            let data2_clone2 = data2_clone.clone();
            println!("Value from Data1 clone2: {}", data1_clone2.value);
            println!("Text from Data2 clone2: {}", data2_clone2.text);
            println!("Thread is going to exit");
        });
    }

    handle.join().unwrap();
    println!("Value from Data1 original: {}", data1.value);
    println!("Text from Data2 original: {}", data2.text);
}

pub fn test() {
    debug_signle_thread();
    debug_multi_thread();
}

#[test]
fn test_debugArc_single_thread() {
    debug_signle_thread();
}

#[test]
fn test_debugArc_multithread() {
    debug_multi_thread();
}