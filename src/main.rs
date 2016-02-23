use std::cmp::Ordering;

//Our node structure
struct Node<V> {
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
    data: V,
}

//New nodes aren't required to be ordinal, only if being found or inserted
impl <V> Node<V> {
    fn new(data: V) -> Node<V> {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }
}

//If we want to insert or look for something, we need to be able to compare against it
impl <V: Ord> Node<V> {
    //Insert nodes into a tree using a specified root
    fn insert(&mut self, n: Node<V>) {
        match n.data.cmp(&self.data) {
            //We can't insert duplicates into BSTs
            Ordering::Equal => {}
            //Navigate or insert down left of tree
            Ordering::Less => {
                match self.left {
                    None => self.left = Some(Box::new(n)),
                    Some(ref mut l) => l.insert(n),
                }
            }
            //Navigate or insert down right of tree
            Ordering::Greater => {
                match self.right {
                    None => self.right = Some(Box::new(n)),
                    Some(ref mut r) => r.insert(n),
                }
            }
        }
    }

    //Determine if a value exists in the tree, return an Option for its existence
    fn get(&self, n: V) -> Option<&V> {
        match n.cmp(&self.data) {
            //We found it, return it
            Ordering::Equal => Some(&self.data),
            //It's less, navigate down the tree
            Ordering::Less => {
                match self.left {
                    None => None,
                    Some(ref l) => l.get(n),
                }
            },
            //It's greater, navigate down the tree
            Ordering::Greater => {
                match self.right {
                    None => None,
                    Some(ref r) => r.get(n),
                }
            }
        }
    }
}

fn main() {
    let mut root = Node::new(5);
    root.insert(Node::new(6));
    root.insert(Node::new(4));
    root.insert(Node::new(3));
    root.insert(Node::new(10));
    match root.left {
        Some(ref l) => {
            println!("Value of root.left: {}", l.data);
            match l.left {
                None => println!("root.left.left is None"),
                Some(ref l) => println!("Value of root.left.left: {}", l.data),
            }
        },
        None => println!("Nothing there"),
    }
    match root.get(10) {
        None => println!("Ten is not in the tree"),
        Some(_) => println!("Ten is in the tree"),
    }
}
