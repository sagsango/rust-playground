
use std::rc::Arc;

#[derive(Debug)]
struct Node {
    name: String,
    id:i64,
}

impl Node {
    fn new(name: String, id:i64) -> Node {
        Node {
            name,
            id,
        }
    }
}


fn main() {
    let mut a:Arc<mut Node> = Arc::new(Node::new(String::from("a"), 1));
    let b = a.clone();


    a.name = String::from("b");
    a.id = 2;



    println!("a: {:?}", a);
    println!("b: {:?}", b);



    


}
