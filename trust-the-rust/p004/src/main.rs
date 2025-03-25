
use std::option::Option;
struct Node {
    name: String,
    age:i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(name:String, age:i32) -> Node {
        Node {
            name:name,
            age:age,
            next:None,
        }
    }
}
struct List {
    head: Option<Box<Node>>,
}

impl List {
    fn new() -> List {
        List {
            head : None,
        }
    }
    fn push_back(&mut self, name:String, age:i32) ->() {
        let new_node = Node::new(name, age);
        if self.head.is_none() {
            self.head = Some(Box::new(new_node));
            return;
        }
        let mut node = self.head.as_mut().unwrap();
        while node.next.is_some() {
            node = node.next.as_mut().unwrap();
        }
        node.next = Some(Box::new(new_node));
    }

    fn push_front(&mut self, name:String, age:i32) ->() {
        let mut new_head = Some(Box::new(Node::new(name, age)));
        new_head.as_mut().unwrap().next  = self.head.take();
        self.head = new_head.take();
    }

    fn pop_back(&mut self) -> Option<Box<Node>> {
        if self.head.is_none() { /* 0 Nodes */
            return None;
        }
        if self.head.as_ref().unwrap().next.is_none() { /* 1 Nodes */
            let back = self.head.take();
            return back;
        }
        let mut cur = self.head.as_mut().unwrap();
        while cur.next.as_ref().unwrap().next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }

        let back = cur.next.take();
        return back;
    }

    fn pop_front(&mut self) -> Option<Box<Node>> {
        if self.head.is_none() {
            return None;
        }
       
        let mut front = self.head.take();
        self.head = front.as_mut().unwrap().next.take();

        return front;
        
    }
    fn show(&self) {
        let mut node = self.head.as_ref();
        while node.is_some() {
            print!("[{}:{}] ->", node.unwrap().name, node.unwrap().age);
            node = node.unwrap().next.as_ref();
        }
        println!("None");
    }
}


fn main() {
    println!("Hello, world!");
    let mut list = List::new();
    for i in 0..5 {
        list.push_back(String::from("Hello"), i);
        list.show();
    }
    for i in 0..5 {
        list.pop_back();
        list.show();
    }

    for i in 0..5 {
        list.push_front(String::from("Hello"), i);
        list.show();
    }
    for i in 0..5 {
        list.pop_front();
        list.show();
    }
}