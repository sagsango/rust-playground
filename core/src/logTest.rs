use log::{debug, error, info, trace, warn};
 use env_logger;

 /*
    * This is a simple example to demonstrate logging in Rust
    * We will use the log crate and env_logger crate
    * How to run:
    * 1. RUST_LOG=debug cargo run
    * 2. RUST_LOG=info cargo run
    * 3. RUST_LOG=warn cargo run
    * 4. RUST_LOG=error cargo run
    * 5. RUST_LOG=trace cargo run
    * 6. RUST_LOG=off cargo run


    // Example:
    ɸ RUST_LOG=trace cargo run
    [2024-09-15T19:43:23Z INFO  rust_playground::logTest] This is an info message
    [2024-09-15T19:43:23Z WARN  rust_playground::logTest] This is a warning message
    [2024-09-15T19:43:23Z DEBUG rust_playground::logTest] This is a debug message
    [2024-09-15T19:43:23Z TRACE rust_playground::logTest] This is a trace message
    [2024-09-15T19:43:23Z ERROR rust_playground::logTest] This is an error message

    RUST_LOG=debug cargo run
    [2024-09-15T19:43:52Z INFO  rust_playground::logTest] This is an info message
    [2024-09-15T19:43:52Z WARN  rust_playground::logTest] This is a warning message
    [2024-09-15T19:43:52Z DEBUG rust_playground::logTest] This is a debug message
    [2024-09-15T19:43:52Z ERROR rust_playground::logTest] This is an error message

    RUST_LOG=info cargo run
    [2024-09-15T19:43:39Z INFO  rust_playground::logTest] This is an info message
    [2024-09-15T19:43:39Z WARN  rust_playground::logTest] This is a warning message
    [2024-09-15T19:43:39Z ERROR rust_playground::logTest] This is an error message

    RUST_LOG=warn cargo run
    [2024-09-15T19:44:13Z WARN  rust_playground::logTest] This is a warning message
    [2024-09-15T19:44:13Z ERROR rust_playground::logTest] This is an error message

    RUST_LOG=error cargo run
    [2024-09-15T19:44:26Z ERROR rust_playground::logTest] This is an error message
  */


pub fn test() {
    // Initialize the logger (this should be called only once in the program)
     env_logger::init();

    // Now, log messages with different log levels
    print!("This is a print message");
    info!("This is an info message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    trace!("This is a trace message");
    error!("This is an error message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        test();
    }
}
