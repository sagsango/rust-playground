use std::io::stdin;
use std::io::Read;

pub fn test() {

    println!("Enter a number in line:");
    /* One numer in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let num:i64 = buf.trim().parse().unwrap();
            println!("Number: {}", num);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter multiple numbers in line: ");
    /* Multiple number in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let nums:Vec<i64> = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            println!("Numbers: {:?}", nums);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter a string: ");
    /* One string in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let s = buf.trim();
            println!("String: {}", s);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter multiple strings in line: ");
    /* Multiple string in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let s:Vec<&str> = buf.trim().split_whitespace().collect();
            println!("Strings: {:?}", s);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter a char in line: ");
    /* One char in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let c = buf.trim().chars().next().unwrap();
            println!("Char: {}", c);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter multiple chars in line: ");
    /* Multiple char in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let c:Vec<char> = buf.trim().chars().collect();
            println!("Chars: {:?}", c);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter a float in line: ");
    /* One float in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let f:f64 = buf.trim().parse().unwrap();
            println!("Float: {}", f);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter multiple floats in line: ");
    /* Multiple float in a line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            let f:Vec<f64> = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            println!("Floats: {:?}", f);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    println!("Enter a line: ");
    /* One line */
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(cnt) => {
            println!("Number of bytes read: {}", cnt);
            println!("Line: {}", buf);
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

}

