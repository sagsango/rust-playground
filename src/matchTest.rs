use std::fs::File;
use std::io::ErrorKind;

// This file is used to test the match statement in Rust
// match used in Enums
// match used in Option
// match used in Result
// Lets see some examples

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

// compex enum
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}


pub fn test() {
    let color = Color::RGB(100, 200, 50);

    /*
    // This can not be used for compaing the enums
    if color == Color::Red {
        println!("Red");
    } else if color == Color::Green {
        println!("Green");
    } else if color == Color::Blue {
        println!("Blue");
    } else {
        println!("RGB");
    }*/

    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
    }

    let msg = Message::Write(String::from("Hello, World!"));
    println!("{:?}", msg);
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move x: {}, y: {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(c) => {
            match c {
                Color::Red => println!("Red"),
                Color::Green => println!("Green"),
                Color::Blue => println!("Blue"),
                Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
            }
        }
    }

    // In case of Result, we can use match to handle the error
    let result: Result<i32, String> = Ok(10);
    match result {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }   


    // In case of Option, we can use match to handle the None
    let option = Some(10);
    match option {
        Some(value) => println!("Value: {}", value),
        None => println!("None"),
    }

    let option: Option<i128> = None;
    match option {
        Some(value) => println!("Value: {}", value),
        None => println!("None"),
    }
    
    // Almost all the function in rust returns Result or Option
    // So, match is very useful in Rust
    // here is an example of match in unwrap opening a non existent file
    let file = std::fs::File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        test();
    }
}