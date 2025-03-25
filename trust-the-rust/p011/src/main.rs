 
use std::rc::Rc;

struct Node {
    data:String,
    next:Option<Box<Node>>,
}

impl Node {
    fn new(data:String) -> Node {
        Node {
            data:data,
            next:None,
        }
    }
}

struct List {
    head:Option<Box<Node>>,
}

impl List {
    fn new() -> List {
        List {
            head : None,
        }
    }
    fn insert(&mut self, data:String) -> () {
        let mut node = Node::new(data); // data is moved
        node.next = self.head.take();
        self.head.replace(Box::new(node));
    }
    fn print(&self) -> () {
        let mut ptr = &self.head;
        while ptr.is_some() {
            print!("{} -> ", ptr.as_ref().unwrap().data);
            ptr = &ptr.as_ref().unwrap().next;
        }
        println!("None");
    }
    fn reverse(&mut self) -> () {
        if self.head.is_none() {
            return;
        }
        let mut cur = self.head.take();
        let mut prv: Option<Box<Node>> = None;
        while cur.is_some() {
            let mut next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = prv.take();
            prv = cur.take();
            cur = next.take();
        }
        self.head = prv.take();
    }
}


fn main() {

    let mut list = List::new();
    let names:Vec<String> = vec![String::from("A"), String::from("B"), String::from("C")];
    for name in names { // moved
        list.insert(name);
    }
    list.print();
    list.reverse();
    list.print();
    println!("Hello, world!");
}
