
use std::{char::TryFromCharError, collections::BTreeSet};

fn main() {
    let arr : Vec<String> = vec![String::from("a"), String::from("b"), String::from("c"), String::from("Hello")];
    let mut set: BTreeSet<String> = BTreeSet::new();

    let len = arr.len();

    for i in 0..len { /* To aoid the move we traverse in the range not in the content */
        println!("{} ", arr[i]); /* print! do not take the args by move */
    }

    for i in 0..len {
        println!("{} ", &arr[i]);
        set.insert(arr[i].clone());
    }

    set.remove(&arr[1]);

    let mut count = 0;
    for i in 0..len { /* To aoid the move we traverse in the range not in the content */
        if set.contains(&arr[i]) {
            count += 1;
        }
        match set.contains(&arr[i]) {
            True=> {
                print!("Hit => ")
            }
            False => {
                print!("Miss => ");
            }
        }

        match set.get(&arr[i]) {
            Some(s)=> {
                println!("{} is present", s);
            }
            _ => {
                println!("{} is not present", &arr[i]);
            }
        }
    }

    println!("Total presnet elements:{}", count);
   
    /* this will move the String from the arr:Vec<String> */
    for name in arr { /* name is mobved from the arr:Vec<Stirng> */
        println!("{} ", name);
        println!("{} ", name);
    }

    println!("Hello, world!");
}
