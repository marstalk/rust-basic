use std::{cell::RefCell, rc::Rc};

pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, next) => Some(next),
            List::Nil => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cyclereference() {
        let a2 = Rc::new(List::Cons(22, RefCell::new(Rc::new(List::Nil))));
        let a1 = Rc::new(List::Cons(11, RefCell::new(a2.clone())));

        let a2_ref = a1.tail();
        if let Some(a2_ref) = a2_ref {
            *a2_ref.borrow_mut() = Rc::clone(&a1);
        }
    }
}
