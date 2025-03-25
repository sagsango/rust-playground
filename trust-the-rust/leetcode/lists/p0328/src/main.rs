use std::collections::LinkedList;


struct Node {
    data:i32,
    next:Option<Box<Node>>,
}

impl Node {
    fn new(data:i32) -> Node {
        Node {
            data:data,
            next:None,
        }
    }
}

struct List {
    head:Option<Box<Node>>
}

impl List {
    fn new() -> List {
        List {
            head:None,
        }
    }
    fn new_with_arg(head:Option<Box<Node>>) -> List {
        List {
            head:head
        }
    }

    fn add(&mut self,mut node:Option<Box<Node>>)  {
        node.as_mut().unwrap().next = self.head.take();
        self.head = node.take();
    }
    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return
        }
        let mut prv:Option<Box<Node>> = None;
        let mut cur:Option<Box<Node>> = self.head.take();
        while cur.is_some() {
            let mut nxt:Option<Box<Node>> = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = prv.take();
            prv = cur.take();
            cur = nxt.take();
        }
        self.head = prv.take();
    }
    fn print(&self) {
        let mut cur = self.head.as_ref();
        while cur.is_some() {
            print!("{}->", cur.unwrap().data);
            cur = cur.unwrap().next.as_ref();
        }
        println!("None");
    }
}


struct Solution {
}
impl Solution {
    fn odd_even_list(mut head : Option<Box<Node>>) -> Option<Box<Node>> {
        let mut p:usize = 0;
        let mut cur: Option<Box<Node>> = head.take();
        let mut lists = [List::new(), List::new()];
        /*
            0, 1, 2, 3
            2, 0
            3, 1

            1, 3

            1, 2, 0
            3, 1, 2, 0

            0, 2, 1, 3
        
        
         */
        while cur.is_some() {
            let mut nxt= cur.as_mut().unwrap().next.take();
            lists[p].add(cur.take());
            p ^= 1;
            cur = nxt.take();
        }
        lists[1].reverse();

        cur = lists[1].head.take();
        while cur.is_some() {
            let mut nxt= cur.as_mut().unwrap().next.take();
            lists[0].add(cur.take());
            cur = nxt.take();
        }
        lists[0].reverse();
        lists[0].head.take()
    }
}

fn main() {
    let mut list = List::new();
    for i in 0..6 {
        list.add(Some(Box::new(Node::new(i))));
        list.print();
    }
    list.reverse();
    list.print();

    let new_list = List::new_with_arg(Solution::odd_even_list(list.head));
    new_list.print();   



    println!("Hello, world!");
}
