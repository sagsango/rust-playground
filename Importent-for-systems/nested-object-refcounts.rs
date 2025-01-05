use std::sync::Arc;

#[derive(Debug)]
struct D {
    name: String,
}

impl Drop for D {
    fn drop(&mut self) {
        println!("\nDrop called for D: {}", self.name);
    }
}

#[derive(Debug)]
struct C {
    name: String,
    d: Arc<D>,
}

impl Drop for C {
    fn drop(&mut self) {
        println!("\nDrop called for C: {}", self.name);
    }
}

#[derive(Debug)]
struct B {
    name: String,
    c: Arc<C>,
}

impl Drop for B {
    fn drop(&mut self) {
        println!("\nDrop called for B: {}", self.name);
    }
}

#[derive(Debug)]
struct A {
    name: String,
    b: Arc<B>,
}

impl Drop for A {
    fn drop(&mut self) {
        println!("\nDrop called for A: {}", self.name);
    }
}

fn print_strong_counts(a: &Arc<A>, b: &Arc<B>, c: &Arc<C>, d: &Arc<D>) {
    println!(
        "Strong counts - a: {}, b: {}, c: {}, d: {}",
        Arc::strong_count(a),
        Arc::strong_count(b),
        Arc::strong_count(c),
        Arc::strong_count(d),
    );
}

fn main() {
    // Create the structures
    let d = Arc::new(D {
        name: "D".to_string(),
    });

    let c = Arc::new(C {
        name: "C".to_string(),
        d: Arc::clone(&d),
    });

    let b = Arc::new(B {
        name: "B".to_string(),
        c: Arc::clone(&c),
    });

    let a = Arc::new(A {
        name: "A".to_string(),
        b: Arc::clone(&b),
    });

    println!("After creation:");
    print_strong_counts(&a, &b, &c, &d);

    // Clone `a`
    let a_clone = Arc::clone(&a);
    println!("\nAfter cloning a:");
    print_strong_counts(&a, &b, &c, &d);

    // Clone `b`
    let b_clone = Arc::clone(&b);
    println!("\nAfter cloning b:");
    print_strong_counts(&a, &b, &c, &d);

    // Clone `c`
    let c_clone = Arc::clone(&c);
    println!("\nAfter cloning c:");
    print_strong_counts(&a, &b, &c, &d);

    // Clone `d`
    let d_clone = Arc::clone(&d);
    println!("\nAfter cloning d:");
    print_strong_counts(&a, &b, &c, &d);

    // Drop `d_clone`
    drop(d_clone);
    println!("\nAfter dropping d_clone:");
    print_strong_counts(&a, &b, &c, &d);

    // Drop `c_clone`
    drop(c_clone);
    println!("\nAfter dropping c_clone:");
    print_strong_counts(&a, &b, &c, &d);

    // Drop `b_clone`
    drop(b_clone);
    println!("\nAfter dropping b_clone:");
    print_strong_counts(&a, &b, &c, &d);

    // Drop `a_clone`
    drop(a_clone);
    println!("\nAfter dropping a_clone:");
    print_strong_counts(&a, &b, &c, &d);

    // Drop original references
    println!("\nGoing to drop ref in backward [d, c, b]:");

    drop(d);
    println!("\nAfter dropping d:");
    print_strong_counts(&a, &b, &c, &a.b.c.d);

    drop(c);
    println!("\nAfter dropping c:");
    print_strong_counts(&a, &b, &a.b.c, &a.b.c.d);

    drop(b);
    println!("\nAfter dropping b:");
    print_strong_counts(&a, &a.b, &a.b.c, &a.b.c.d);

    // Clone and drop `a`
    let a_clone = Arc::clone(&a);
    println!("\nAfter cloning a:");
    print_strong_counts(&a, &a.b, &a.b.c, &a.b.c.d);

    drop(a_clone);
    println!("\nAfter dropping a_clone:");
    print_strong_counts(&a, &a.b, &a.b.c, &a.b.c.d);

    // Drop `a`
    println!("\nNOTE: In Rust, the Drop trait is called only when the last reference to a resource is dropped.");
    
    println!("\nGoing to drop all originals:");
    drop(a);
    println!("\nAfter dropping all originals:");
}
