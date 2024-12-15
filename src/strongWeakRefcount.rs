use std::sync::{Arc, Weak};

pub fn test() {
    // Create an Arc, with strong count = 1 and weak count = 0
    let strong = Arc::new(42);
    println!("Strong count: {}", Arc::strong_count(&strong)); // 1
    println!("Weak count: {}", Arc::weak_count(&strong)); // 0

    // Create a weak reference
    let weak: Weak<i32> = Arc::downgrade(&strong);
    println!("Strong count: {}", Arc::strong_count(&strong)); // 1
    println!("Weak count: {}", Arc::weak_count(&strong)); // 1

    // Clone the strong reference
    let strong_clone = Arc::clone(&strong);
    println!("Strong count: {}", Arc::strong_count(&strong)); // 2
    println!("Weak count: {}", Arc::weak_count(&strong)); // 1

    // Drop one strong reference
    drop(strong_clone);
    println!("Strong count: {}", Arc::strong_count(&strong)); // 1
    println!("Weak count: {}", Arc::weak_count(&strong)); // 1

    // Drop the last strong reference
    drop(strong);
    // The data is deallocated, but the weak reference still exists.
    // Attempting to upgrade the weak reference will fail.
    if let Some(data) = weak.upgrade() {
        println!("Upgraded weak reference: {}", data);
    } else {
        println!("The strong references are gone, data is deallocated.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strong_weak_refcount() {
        test();
    }
}

