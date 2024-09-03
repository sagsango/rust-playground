//! This module contains the configuration options for the application.
//! source code: https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/config.rs
//! # Examples:
//! ```
//! use config::Logging;
//! let config = Logging::new();
//! ```
//! 

use std::fs::OpenOptions;
use std::io::{self, Write};  // Import the Write trait
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn log(&self, message: &str) {
        if self.enabled {
            match self.destination {
                LogOutput::Stdout => println!("{}", message),
                LogOutput::Stderr => eprintln!("{}", message),
                LogOutput::File(ref file) => {
                    let mut file = std::fs::OpenOptions::new().append(true).create(true).open(file).unwrap();
                    writeln!(file, "{}", message).unwrap();// error : must implement `io::Write`, `fmt::Write`, or have a `write_fmt` method

                }
            }
        }
    }
}

pub fn test() {
    let mut config = Logging::new();
    config.enable();
    config.set_level(LogLevel::Debug);
    config.set_destination(LogOutput::Stdout);
    config.log("This is a debug message");
    config.set_level(LogLevel::Info);
    config.log("This is an info message");
    config.set_level(LogLevel::Warn);
    config.log("This is a warning message");
    config.set_level(LogLevel::Error);
    config.log("This is an error message");
    config.set_destination(LogOutput::Stderr);
    config.log("This is an error message");
    config.set_destination(LogOutput::File("log.txt".to_string()));
    config.log("This is an error message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
       test();
    }
}
