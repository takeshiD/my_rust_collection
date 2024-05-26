use std::rc::Rc;
use std::cell::RefCell;

struct Node<T: Ord> {
    data: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

struct BinaryTree<T: Ord> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord> Node<T> {
    fn new(elt: T) -> Self {
        Node {
            data: elt,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, elt: T) {
        if elt == self.data { return; }
        let node = if elt < self.data {
            &mut self.left
        } else {
            &mut self.right
        };
        match node {
            &mut Some(ref mut node1) => node1.borrow_mut().insert(elt),
            &mut None => *node = Some(Rc::new(RefCell::new(Node::new(elt)))),
        }
    }
    fn lhs(&self) -> Option(&Node<T>) {

    }
}
impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree {
            root: None
        }
    }
    fn insert(&mut self, elt: T) {
        unimplemented!()
    }
    fn search(&self, elt: T) -> bool {
        unimplemented!()
    }
    fn remove(&self, elt: T) -> Option<T> {
        unimplemented!()
    }
    fn search_min(&self) -> Option<T> {
        unimplemented!()
    }
    fn remove_min(&mut self) -> Option<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node_insert() {
        let mut node = Node::new(5);
        node.insert(3);
        node.insert(6);
        if let Some(left) = node.left {
            let left_borrow = left.borrow();

        }
    }
    #[test]
    fn test_bintree_insert() {
        let mut btree = BinaryTree::<i32>::new();
        btree.insert(10);
        btree.insert(14);
        btree.insert(7);
        btree.insert(3);
        btree.insert(12);
    }
    #[test]
    fn test_bintree_search_min() {
        let mut btree = BinaryTree::<i32>::new();
        btree.insert(10);
        btree.insert(14);
        btree.insert(7);
        btree.insert(3);
        btree.insert(12);
        assert_eq!(btree.search_min(), Some(3));
    }
    #[test]
    fn test_bintree_search() {
        let mut btree = BinaryTree::<i32>::new();
        btree.insert(10);
        btree.insert(14);
        btree.insert(7);
        btree.insert(3);
        btree.insert(12);
        assert_eq!(btree.search(7), true);
        assert_eq!(btree.search(-10), false);
    }
    #[test]
    fn test_bintree_remove() {
        let mut btree = BinaryTree::<i32>::new();
        btree.insert(10);
        btree.insert(14);
        btree.insert(7);
        btree.insert(3);
        btree.insert(12);
        assert_eq!(btree.search(7), true);
        assert_eq!(btree.search(-10), false);
    }
}
