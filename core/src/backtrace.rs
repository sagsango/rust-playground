use std::backtrace::Backtrace;

pub fn test() {
    let backtrace = Backtrace::force_capture();
    println!("Captured Backtrace:\n{:#?}", backtrace); // Pretty-print the backtrace
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrace() {
        test();
    }
}