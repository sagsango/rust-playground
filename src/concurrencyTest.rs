// concurrency tests
// This file will deal with threads, data sharing, mutexes, and channels
use std::thread;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering}; 
use std::mem::size_of;
use memoffset::offset_of;
extern crate spin;

pub fn mutex_test1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn mutex_test2() {
    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let tx = tx.clone();
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            tx.send(()).unwrap();
        });
        handles.push(handle);
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// mutex test 3 will deal with the mutex guard around the struct, and use this method to update the struct
// struct data member will be guarded by the mutex guard
// struct will be shared between threads
struct Person {
    name: String,
    age: u8,
    // data member guarded by the mutex guard
    counter: Mutex<u32>,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age,
            counter: Mutex::new(0),
        }
    }
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
    fn increment(&self) {
        let mut num = self.counter.lock().unwrap();
        *num += 1;
    }
    fn get_counter(&self) -> u32 {
        *self.counter.lock().unwrap()
    }
}

fn mutex_test3() {
    let person = Arc::new(Person::new("John".to_string(), 32));
    let mut handles = vec![];

    for _ in 0..10 {
        let person = Arc::clone(&person);
        let handle = thread::spawn(move || {
            println!("Thread: {:?}, counter: {}", thread::current(), person.get_counter());
            person.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    person.print();
    println!("Final counter: {}", person.get_counter());
}

// this time struct will be having a data member with mutex guard and struct will be using padding to avoid false sharing
struct Person2 {
    name: String,
    age: u8,
    // data member guarded by the mutex guard
    counter: Mutex<u32>,
    // padding to avoid false sharing
    _padding: [u8; 64],
}

impl Person2 {
    fn new(name: String, age: u8) -> Self {
        Person2 {
            name,
            age,
            counter: Mutex::new(0),
            _padding: [0; 64],
        }
    }
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
    fn increment(&self) {
        let mut num = self.counter.lock().unwrap();
        *num += 1;
    }
    fn get_counter(&self) -> u32 {
        *self.counter.lock().unwrap()
    }
}

fn mutex_test4() {
    let person = Arc::new(Person2::new("John".to_string(), 32));
    // print size of the struct and offset of evey data member
    println!("Size of Person2: {}", std::mem::size_of::<Person2>());
    println!("Size of name: {}", std::mem::size_of::<String>());
    println!("Size of age: {}", std::mem::size_of::<u8>());
    println!("Size of counter: {}", std::mem::size_of::<Mutex<u32>>());
    println!("Size of padding: {}", std::mem::size_of::<[u8; 64]>());
    // print offset of every data member
    println!("Offset of name: {}", offset_of!(Person2, name));
    println!("Offset of age: {}", offset_of!(Person2, age));
    println!("Offset of counter: {}", offset_of!(Person2, counter));
    println!("Offset of padding: {}", offset_of!(Person2, _padding));
   

    let mut handles = vec![];

    for _ in 0..10 {
        let person = Arc::clone(&person);
        let handle = thread::spawn(move || {
            println!("Thread: {:?}, counter: {}", thread::current(), person.get_counter());
            person.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    person.print();
    println!("Final counter: {}", person.get_counter());
}


// this time we will use spinlock to avoid the overhead of mutex, togather with padding to avoid false sharing
// spinlock is a busy waiting lock
// it will keep on checking the lock until it is free
// it is useful when the lock is held for a very short time
// it is not useful when the lock is held for a long time
struct Person3 {
    name: String,
    age: u8,
    // data member guarded by the mutex guard
    counter: spin::Mutex<u32>,
    // padding to avoid false sharing
    _padding: [u8; 64],
}

impl Person3 {
    fn new(name: String, age: u8) -> Self {
        Person3 {
            name,
            age,
            counter: spin::Mutex::new(0),
            _padding: [0; 64],
        }
    }
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
    fn increment(&self) {
        let mut num = self.counter.lock();
        *num += 1;
    }
    fn get_counter(&self) -> u32 {
        *self.counter.lock()
    }
}

fn mutex_test5() {
    let person = Arc::new(Person3::new("John".to_string(), 32));
    // print size of the struct and offset of evey data member
    println!("Size of Person3: {}", std::mem::size_of::<Person3>());
    println!("Size of name: {}", std::mem::size_of::<String>());
    println!("Size of age: {}", std::mem::size_of::<u8>());
    println!("Size of counter: {}", std::mem::size_of::<spin::Mutex<u32>>());
    println!("Size of padding: {}", std::mem::size_of::<[u8; 64]>());
    // print offset of every data member
    println!("Offset of name: {}", offset_of!(Person3, name));
    println!("Offset of age: {}", offset_of!(Person3, age));
    println!("Offset of counter: {}", offset_of!(Person3, counter));
    println!("Offset of padding: {}", offset_of!(Person3, _padding));
   

    let mut handles = vec![];

    for _ in 0..10 {
        let person = Arc::clone(&person);
        let handle = thread::spawn(move || {
            println!("Thread: {:?}, counter: {}", thread::current(), person.get_counter());
            person.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    person.print();
    println!("Final counter: {}", person.get_counter());
}

// barrier test
fn barrier_test1() {
    let num_threads = 10;
    let barrier = Arc::new(std::sync::Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread: {:?}, before wait", thread::current());
            barrier.wait();
            println!("Thread: {:?}, after wait", thread::current());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// this barrier test will test the barrier with a spinlock
fn barrier_test2() {
    let num_threads = 10;
    let barrier = Arc::new(spin::Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread: {:?}, before wait", thread::current());
            barrier.wait();
            println!("Thread: {:?}, after wait", thread::current());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// atomic test
fn atomic_test1() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let num = counter.fetch_add(1, Ordering::SeqCst);
            println!("Thread: {:?}, counter: {}", thread::current(), num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", counter.load(Ordering::SeqCst));
}


// this will do compaer and swap
fn atomic_test2() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.load(Ordering::SeqCst);
            loop {
                let old_num = num;
                let new_num = old_num + 1;
                match counter.compare_exchange(old_num, new_num, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(_) => {
                        println!("Thread: {:?}, counter: {}", thread::current(), new_num);
                        break;
                    }
                    Err(x) => {
                        println!("Thread: {:?}, Unexpected counter: {}", thread::current(), x);
                        num = x;
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", counter.load(Ordering::SeqCst));
    
}


pub fn mutex_test() {
    mutex_test1();
    mutex_test2();
    mutex_test3();
    mutex_test4(); // sleeplock test
    mutex_test5(); // spinlock test
}

fn barrier_test() {
    barrier_test1();
    barrier_test2();
}

fn atomic_test() {
    atomic_test1();
    atomic_test2();
}


/// Test the concurrency functionality
/// We are testing the following:
/// - mutex_test
/// - barrier_test
/// - atomic_test
/// 
pub fn test() {
    mutex_test();
    barrier_test();
    atomic_test();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mutex_tests() {
        mutex_test();
    }

    #[test]
    fn barrier_tests() {
        barrier_test();
    }

    #[test]
    fn atomic_tests() {
        atomic_test();
    }
}