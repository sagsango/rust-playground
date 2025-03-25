use std::collections::BTreeMap;
use std::vec;
use std::string;

fn main() {

    let arr: Vec<String> = vec![String::from("hi"), String::from("hello"), String::from("Bye")];
    let mut map:BTreeMap<String, usize> = BTreeMap::new();

    let len = arr.len();

    for i in 0..len {
        map.insert(arr[i].clone(), i);
    }


    let brr:Vec<String> = vec![String::from("yy"), String::from("y"), String::from("x"), String::from("X"), String::from("hello"), String::from("A")];

    let len = brr.len();

    for i in 0..len {
        match map.contains_key(&brr[i]) {
            True=> {
                println!("{} is present", &brr[i]);
                /*
                    TODO: Report a bug
                          It says eventhing is presnt.
                 */
            }
            False=> {
                println!("{} is not present", &brr[i]);
            }
        }
    }

    for i in 0..len {
        match map.get(&brr[i]) {
            Some(x) => {
                println!("Removing : [{}:{}]", &brr[i], x);
                map.remove(&brr[i]);
            }
            _ => {

            }
        }
    }


    println!("Hello, world!");
}
