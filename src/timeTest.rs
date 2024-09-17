// Use of time::Instant and time::Duration
use std::time::{Instant, Duration};

pub fn test() {
    // Measure time elapsed for a block of code
    let start = Instant::now();
    std::thread::sleep(Duration::from_secs(2));
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);

    // Convert Duration to seconds and milliseconds
    let secs = elapsed.as_secs();
    let millis = elapsed.as_millis();
    println!("Seconds: {}, Milliseconds: {}", secs, millis);

    // Convert Duration to floating point seconds
    let secs_float = elapsed.as_secs_f64();
    println!("Seconds (float): {}", secs_float);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        test();
    }
}