// @santoshn: An implementation of a binary search tree
// Experiments with the borrow checker

use BST::*;
use std::cmp::Ordering;

enum BST{
    Null,
    Leaf(i32),
    Node(i32, Box<BST>, Box<BST>),
}

fn insert_node (val: i32, tree: BST) -> BST {
    match tree {

        Null => Leaf(val),
        Leaf(curr) => match val.cmp(&curr) {

            Ordering::Greater => Node(curr, Box::new(Null),
                                      Box::new(Leaf(val))),
                                      
            
            Ordering::Less => Node (curr, Box::new(Leaf(val)),
                                    Box::new(Null)),

            Ordering::Equal => panic!("node already present"),
        },
        
        Node(curr, left, right) => match val.cmp(&curr) {

            Ordering::Greater => Node(curr, left,
                                      Box::new(insert_node(val, *right))),

            Ordering::Less => Node(curr,
                                   Box::new(insert_node(val, *left)), right),

            Ordering::Equal => panic!("node already present"),
        },
    }
}

fn inorder_traversal(tree: &BST) {

    match *tree {
        Null => (),
        Leaf(val) => println!(" {} ", val),
        Node(val, ref left, ref right) => { inorder_traversal(&*left);
                                    println!(" {} ", val);
                                    inorder_traversal(&*right);
        },
    }
}

fn preorder_traversal(tree: &BST) {

    match *tree {
        Null =>  (),
        Leaf(val) => println!(" {} ", val),
        Node(val, ref left, ref right) => { println!(" {}", val);
                                            preorder_traversal(&*left);
                                            preorder_traversal(&*right);
        },
    }

}

fn main() {

    let my_vec = [6, 3, 23,1, 5, 72, 42, 16, 19];
    //let my_vec = [6, 3, 21];
    
    let mut my_tree = Null;

    for x in &my_vec {
        my_tree = insert_node(*x, my_tree);
    }
    
    println!("Inorder traversal:");
    inorder_traversal(&my_tree);
    
    println!("Preorder traversal:");
    preorder_traversal(&my_tree);    
}
