//Authors: Kunal Pednekar, santoshn
//Another implementation of an integer binary search tree in Rust
//Works with Rust 1.10

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right:Option<Box<Node>>,
}

fn create_node(new_val:i32)-> Option<Box<Node>> {
    let new_node = Node {val: new_val, left: None, right: None};
    Some(Box::new(new_node))
}

impl Node{
    fn insert(&mut self, new_val: i32){
        if new_val == self.val {
            println!("val already exits");
            return;
        }
        let current = if new_val < self.val { &mut self.left }
                      else {&mut self.right };
        match current {
            &mut None => {
                *current = create_node(new_val);
            }
            &mut Some(ref mut node) => node.insert(new_val),            
        }
    }

    fn traverse(&mut self) {
        let left = &mut self.left;
        match left {
            &mut None => {},
            &mut Some(ref mut node) => {
                node.traverse();
            }
        }
        println!("this val: {}", self.val);
        let right =&mut self.right;
        match right {
            &mut None => {},
            &mut Some(ref mut node) => {
                node.traverse();
            },
        }
    }
}

fn tree_traverse(tree: &mut Option<Box<Node>>) {
    match *tree {
        None => (),
        Some(ref mut node) => {node.traverse();()},
    }
}

fn tree_insert(tree:&mut Option<Box<Node>>, val:i32){
    match *tree{
        None => {*tree = create_node(val);},
        Some(ref mut node) => { node.insert(val);}
    }    
}

fn main() {

    let mut temp: Option<Box<Node>>= None;
    let mut tree: &mut Option<Box<Node>> = &mut temp;

    let my_vec = [6, 3, 23,1, 5, 72, 42, 16, 19];
    for x in &my_vec {
        tree_insert(tree, *x);
    }
   tree_traverse(tree);
    println!("Hello, world!");
}
