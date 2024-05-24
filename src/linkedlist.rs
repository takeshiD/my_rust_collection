use std::rc::Rc;
use std::cell::{RefCell, Ref};

// #[derive(Debug)]
#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

// #[derive(Debug)]
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
        self.len += 1;
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
        self.len += 1;
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let head = self.head.clone().unwrap();
            self.head = head.clone().borrow().next.clone();
            self.len -= 1;
            Some(head.clone().borrow().data.clone())
        }
    }
    fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
        // if self.tail.is_none() {
        //     None
        // } else {
        //     let tail = self.tail.clone().unwrap();
        //     self.tail = tail.clone().borrow().next.clone();
        //     Some(tail.clone().borrow().data.clone())
        // }
    }
    fn front(&self) -> Option<T> {
        match self.head.clone() {
            Some(head) => Some(head.borrow().data.clone()),
            None => None,
        }
    }
    fn back(&self) -> Option<T> {
        match self.tail.clone() {
            Some(tail) => Some(tail.borrow().data.clone()),
            None => None,
        }
    }
    fn len(&self) -> usize {
        self.len.clone()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
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
        assert_eq!(list.len(), 0);
        let n = 10;
        for i in 0..n {
            list.push_back(i);
        }
        assert_eq!(list.len(), n);
    }
    #[test]
    fn test_front(){
        let mut list = LinkedList::new();
        assert_eq!(list.front(), None);
        list.push_front(1);
        assert_eq!(list.front(), Some(1));
        list.push_front(2);
        assert_eq!(list.front(), Some(2));
    }
    #[test]
    fn test_back(){
        let mut list = LinkedList::new();
        assert_eq!(list.back(), None);
        list.push_back(1);
        assert_eq!(list.back(), Some(1));
        list.push_back(2);
        assert_eq!(list.back(), Some(2));
    }
    #[test]
    fn test_is_empty(){
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
        list.pop_front();
        assert!(list.is_empty());
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
        let mut list = LinkedList::new();
        {
            let front = list.pop_front();
            assert!(front.is_none());
        }
        let n = 5;
        for i in 0..n {
            list.push_back(i);
        }
        for i in 0..n {
            let front = list.pop_front();
            assert_eq!(front, Some(i));
        }
        {
            let front = list.pop_front();
            assert!(front.is_none());
        }
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
    #[ignore]
    fn test_pop_back(){
        unimplemented!()
        // let mut list = LinkedList::new();
        // {
        //     let back = list.pop_back();
        //     assert!(back.is_none());
        // }
        // let n = 5;
        // for i in 0..n {
        //     list.push_front(i);
        // }
        // for i in 0..n {
        //     let back = list.pop_back();
        //     assert_eq!(back, Some(i));
        // }
        // {
        //     let back = list.pop_back();
        //     assert!(back.is_none());
        // }
    }
}
