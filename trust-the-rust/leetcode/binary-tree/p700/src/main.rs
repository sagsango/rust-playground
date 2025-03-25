use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

struct Bst {
    root: Option<Rc<RefCell<Node>>>,
}

impl Bst {
    fn new() -> Bst {
        Bst { root: None }
    }

    // Adjusted insert to take &mut self and use self.root directly
    fn insert(&mut self, val: i32) {
        Self::insert_node(&mut self.root, Node::new(val));
    }

    // Helper method to recursively insert a node
    fn insert_node(root: &mut Option<Rc<RefCell<Node>>>, node: Node) {
        if root.is_none() {
            root.replace(Rc::new(RefCell::new(node)));
            return;
        }

        // This is the better and clean way
        /* 
        let mut node_ref = root.as_mut().unwrap().borrow_mut();
        if node_ref.val < node.val {
            Self::insert_node(&mut node_ref.right, node);
        } else {
            Self::insert_node(&mut node_ref.left, node);
        }
        */



			  // NOTE: We use borrow to increase the refcount.
				//       Because a node is pointing is curr node, 
				//       and also we are passing curr node to the insert_root()



        // This is my way
        if root.as_ref().unwrap().borrow().val > node.val {
            //Self::insert_node(&mut node_ref.right, node);
            Self::insert_node(&mut root.as_mut().unwrap().borrow_mut().left, node);
        } else {
            //Self::insert_node(&mut node_ref.left, node);
            Self::insert_node(&mut root.as_mut().unwrap().borrow_mut().right, node);
        }
    }

    fn print(&self) {
        Self::print_node(&self.root);
    }

    fn print_node(root: &Option<Rc<RefCell<Node>>>) {
        match root {
            Some(node) => {
                let node = node.borrow();
                Self::print_node(&node.left);
                print!("{} ", node.val);
                Self::print_node(&node.right);
            }
            _ => {
                
            }
        }
    }

    fn tree_nodes(root:& Option<Rc<RefCell<Node>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let l = Self::tree_nodes(& root.as_ref().unwrap().borrow().left);
        let r = Self::tree_nodes(& root.as_ref().unwrap().borrow().right);
        l + r + 1
    }

    fn count_nodes(&self) -> i32 {
        return Self::tree_nodes(&self.root);
    }
}

fn main() {
    let mut bst = Bst::new();
    let keys = vec![10, 1, 2, 9, 5, 7, 6, 4, 3, 8];
    for key in keys {
        bst.insert(key);
        bst.print();
        println!(); // Newline for readability
        println!("#nodes: {}", bst.count_nodes());
    }
    println!("Hello, world!");
}
