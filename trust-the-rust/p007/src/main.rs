
use std::vec;

struct Node {
    a:i32,
    b:i32,
    name:String,
}

fn print_node_vec(arr:& Vec<Node>) -> () {
    print!("Arr: ");
    for node in arr {
        print!("[{} {} {}] ", node.a, node.b, node.name);
    }
    println!("");
}

fn main() {
    println!("Hello, world!");

    let mut arr:Vec<Node> = Vec::new();

    for a in 0..5 {
        for b in 0..5 {
            let name:String = String::from("Hello");
            arr.push(Node {a:5-a, b:5-b, name:name});
        }
    }

    print_node_vec(&arr);

    arr.sort_by(|a, b|{
            if a.a != b.a {
                return a.a.cmp(&b.a);
            }
            if a.b != b.b {
                return a.b.cmp(&b.b);
            }
            return a.name.cmp(&b.name);
    });

    print_node_vec(&arr);

    let n  = arr.len();
    for _ in 0..n {
        match arr.pop() {
            Some(Node{a, b:_, name:_}) => {
                print!("{}", a);
            }
            _ => {
                print!("Empty");
            }
        }
    }

    println!("Done");
}
