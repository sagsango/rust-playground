#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>, // Box ensures heap allocation
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node { value, next: self.head.take() });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.head.is_none() {
            return None; // Nothing to pop
        }
    
        let mut old_head = self.head.take().unwrap(); // Take the head node
        let value = old_head.value;
        self.head = old_head.next.take(); // Move ownership of the next node to head

        Some(value) // Return the value of the popped node
    }

    fn print(&self) {
        let mut current = &self.head;
        while current.is_some() {
            let node_val = &current.as_ref().unwrap().value;
            print!("{} -> ", node_val);
            current = &current.as_ref().unwrap().next;
        }
        println!("None");
        /* 
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_deref();
        }
        println!("None");
        */
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    list.print(); // 3 -> 2 -> 1 -> None
    
    list.pop();
    list.print(); // 2 -> 1 -> None
}
