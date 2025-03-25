
use std::sync::Arc;
use std::sync::Mutex;
struct Node {
    prev: Option<Arc<Mutex<Box<Node>>>>,
    next: Option<Arc<Mutex<Box<Node>>>>,
    name:String,
    age:i32,
}

impl Node {
    fn new(name:String, age:i32) -> Node {
        Node {
            prev: None,
            next: None,
            name:name,
            age:age,
        }
    }
    fn print(&self) -> () {
        print!("[{}:{}]->", self.name, self.age);
    }
}

struct List {
    head: Option<Arc<Mutex<Box<Node>>>>
}

impl List {
    fn new() -> List {
        List {
            head: None
        }
    }
    fn push_front(&mut self, name:String, age:i32) -> () {
        let mut node = Some(Arc::new(Mutex::new(Box::new(Node::new(name, age)))));
        if self.head.is_none() {
         self.head = node;
         return;   
        }
        self.head.as_mut().unwrap().lock().unwrap().prev = Some(node.as_ref().unwrap().clone());
        node.as_mut().unwrap().lock().unwrap().next = Some(self.head.as_ref().unwrap().clone());
        self.head = node;
    }

    fn print(&self) -> () {
        /*let mut cur = self.head.as_ref();
        while cur.is_some() {
            /*let node = cur.unwrap(); // Get the current node
            let node_lock = node.lock().unwrap(); // Lock the node
            print!("[{}:{}]->", &node_lock.name, &node_lock.age);
            cur = node_lock.next.as_ref(); // Move to the next node*/
            print!("[{}:{}]->", cur.unwrap().lock().unwrap().name, cur.unwrap().lock().unwrap().age);
            cur = cur.unwrap().lock().unwrap().next.as_ref();
        }*/

        /*
        let mut current = self.head.clone();

        while let Some(node_arc) = current {
            // Lock the Mutex and get the Node data
            let node = node_arc.lock().unwrap();
            
            // Print the node's name and age
            print!("[Name:{}, Age:{}]->", node.name, node.age);

            // Move to the next node
            current = node.next.clone();
        }
        */
        let mut cur = self.head.clone();
        while cur.is_some() {
           cur.as_ref().unwrap().lock().unwrap().print();
           let next = cur.as_ref().unwrap().lock().unwrap().next.clone();
           cur = next;
            
        }     
        println!("None");
    }
    fn push_back(&self, name:String, age:i32) -> () {
        ()
    }
    fn pop_back(&self) -> Option<Arc<Mutex<Box<Node>>>> {
        None
    }
    fn pop_front(&self) -> Option<Arc<Mutex<Box<Node>>>> {
        None
    }
}


fn main() {
    println!("Hello, world!");
    let mut l = List::new();
    for i in 0..5 {
        l.push_front(String::from("Hello"), i);
        l.print();
    }
}
