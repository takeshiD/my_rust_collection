use std::rc::Rc;
use std::cell::RefCell;

enum ConsCell<T> {
    Nil,
    Cons{
        car: T,
        cdr: Rc<RefCell<ConsCell<T>>>
    },
}

impl<T> ConsCell<T> {
    fn new() -> Self {
        ConsCell::Nil
    }
    fn cons(car: T, cdr: ConsCell) -> ConsCell {
        Cons{
            car: car,
            cdr: cdr,
        }
    }
}
