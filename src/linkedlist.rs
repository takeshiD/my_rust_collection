use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Clone> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
    fn push_front(&mut self, elt: T) {
        let new_node = Node {
            data: elt,
            next: None,
        };
        if self.head.is_none() {
            let new_node = Rc::new(RefCell::new(new_node));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            let new_node = Rc::new(RefCell::new(new_node));
            new_node.borrow_mut().next = self.head.clone();
            self.head = Some(new_node);
        }
    }
    fn push_back(&mut self, elt: T) {
        let new_node = Node {
            data: elt,
            next: None,
        };
        if self.head.is_none() {
            let new_node = Rc::new(RefCell::new(new_node));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            let new_node = Rc::new(RefCell::new(new_node));
            if let Some(tail) = &mut self.tail {
                tail.borrow_mut().next = Some(new_node.clone());
            }
            self.tail = Some(new_node);
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }
    fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }
    fn front(&self) -> Option<&T> {
        unimplemented!()
    }
    fn back(&self) -> Option<&T> {
        unimplemented!()
    }
    fn len(&self) -> usize {
        unimplemented!()
    }
    fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_new(){
        let list = LinkedList::<u32>::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.len, 0);
    }
    #[test]
    fn test_len(){
        let mut list = LinkedList::new();
        let n = 10;
        for i in 0..n {
            list.push_back(i);
        }
        assert_eq!(list.len(), n);
        for _ in 0..n {
            list.pop_back();
        }
        assert_eq!(list.len(), 0);
    }
    #[test]
    fn test_front(){
        unimplemented!();
    }
    #[test]
    fn test_back(){
        unimplemented!();
    }
    #[test]
    fn test_is_empty(){
        unimplemented!();
    }
    #[test]
    fn test_push_front(){
        let mut list = LinkedList::new();
        list.push_front(1);
        {
            let head = list.head.clone().unwrap();
            let tail = list.tail.clone().unwrap();
            assert!(std::ptr::eq(head.as_ptr(), tail.as_ptr()));
            assert_eq!(head.borrow().data, 1);
            assert_eq!(tail.borrow().data, 1);
        }
        list.push_front(2);
        {
            let head = list.head.clone().unwrap();
            let tail = list.tail.clone().unwrap();
            assert!(!std::ptr::eq(head.as_ptr(), tail.as_ptr()));
            assert_eq!(head.borrow().data, 2);
            assert_eq!(tail.borrow().data, 1);
        }
    }
    #[test]
    fn test_pop_front(){
        unimplemented!();
    }
    #[test]
    fn test_push_back(){
        let mut list = LinkedList::new();
        list.push_back(1);
        {
            let head = list.head.clone().unwrap();
            let tail = list.tail.clone().unwrap();
            assert!(std::ptr::eq(head.as_ptr(), tail.as_ptr()));
            assert_eq!(head.borrow().data, 1);
            assert_eq!(tail.borrow().data, 1);
        }
        list.push_back(2);
        {
            let head = list.head.clone().unwrap();
            let tail = list.tail.clone().unwrap();
            assert!(!std::ptr::eq(head.as_ptr(), tail.as_ptr()));
            assert_eq!(head.borrow().data, 1);
            assert_eq!(tail.borrow().data, 2);
        }
    }
    #[test]
    fn test_pop_back(){
        unimplemented!();
    }
}
