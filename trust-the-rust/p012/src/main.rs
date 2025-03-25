


use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {

    let mut obj:Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("Hello")));
    let mut cloned_obj  = obj.clone();

    std::thread::spawn(
        move || {
            let mut data = cloned_obj.lock().unwrap();
            *data = String::from("Hi");
        }
    ).join();

    println!("obj: {}", obj.lock().unwrap());
}
