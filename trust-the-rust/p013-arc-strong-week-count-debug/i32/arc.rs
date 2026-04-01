use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(255i32);
    let b = Arc::clone(&a);
    let c = Arc::clone(&a);

    println!("PID: {}", std::process::id());
    println!("a = {:p}", Arc::as_ptr(&a));
    println!("b = {:p}", Arc::as_ptr(&b));
    println!("c = {:p}", Arc::as_ptr(&c));

    println!("strong_count = {}", Arc::strong_count(&a));
    println!("weak_count   = {}", Arc::weak_count(&a));

    // Keep variables alive and give time to attach gdb
    let _keep = (&a, &b, &c);

    println!("Sleeping for 300 seconds... attach gdb now");
    loop { } 
//    thread::sleep(Duration::from_secs(300));

    // Use them again so optimizer cannot drop them too early
    println!("values: {}, {}, {}", *a, *b, *c);
}
