use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T: Eq + Display + Clone> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: Eq + Display + Clone> {
    head: Option<Box<Node<T>>>,
}

impl<T: Eq + Display + Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }
    pub fn reverse_imperative(&mut self) {
        let mut current = self.head.take();
        let mut prev = None;
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            current = next;
            prev = Some(node);
        }
        self.head = prev;
    }

    pub fn reverse_recursive(&mut self) {
        // ownership of head moved.
        let mut head = self.head.take();
        if let Some(node) = head.take() {
            // if head is not none.
            self.reverse_recursive_helper(Some(node), None);
        }
    }
    fn reverse_recursive_helper(
        &mut self,
        mut current: Option<Box<Node<T>>>,
        prev: Option<Box<Node<T>>>,
    ) {
        if let Some(mut current_box) = current.take() {
            let next_node = current_box.next.take();
            current_box.next = prev;
            self.reverse_recursive_helper(next_node, Some(current_box));
        } else {
            // if current node is None which means it's the end of linked list, so set prev (the last node) as the new head.
            self.head = prev;
        }
    }

    pub fn remove_head(&mut self) -> Option<T> {
        let head_node = self.head.take();
        match head_node {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }

    pub fn remove_tail(&mut self) -> Option<T> {
        if let None = self.head {
            return None;
        }

        if let None = self.head.as_ref().unwrap().next {
            return Some(self.head.take().unwrap().val);
        }

        let mut pre = self.head.as_mut().unwrap();

        while let Some(ref mut node) = pre.next {
            if node.next.is_none() {
                break;
            }
            // unwrap return &mut Box<Node> which can be assign to &mut Node
            pre = pre.next.as_mut().unwrap();
        }

        let res = pre.next.take();
        match res {
            Some(node) => Some(node.val),
            None => None,
        }
    }

    /**
     * remove target element if exist.
     */
    pub fn remove_first_hit(&mut self, val: T) {
        if self.head.is_none() {
            return;
        }

        if let Some(ref mut head) = self.head {
            if head.val == val {
                self.head = head.next.take();
                return;
            }
        }

        let mut pre = self.head.as_mut().unwrap();
        while let Some(ref mut node) = pre.next {
            if node.val == val {
                pre.next = node.next.take();
                break;
            }
            // compile failed, because the node is partially mutable borrow from pre.
            // so the pre can't change.
            // pre = node.deref_mut();
            pre = pre.next.as_mut().unwrap();
        }
    }

    pub fn remove_all_hit(&mut self, val: T) {
        if self.head.is_none() {
            return;
        }

        while let Some(ref mut head) = self.head {
            if head.val == val {
                self.head = head.next.take();
            }
            break;
        }

        let mut pre = self.head.as_mut().unwrap();
        // if next is not none.
        while let Some(ref mut node) = pre.next {
            if node.val == val {
                pre.next = node.next.take();
            } else {
                // let a = &pre.next;
                // let b = a.as_mut();
                // let pre = b.unwrap();
                pre = pre.next.as_mut().unwrap();
            }
        }
    }

    pub fn push_head(&mut self, val: T) -> &mut Self {
        let node = Box::new(Node {
            val: val,
            next: self.head.take(),
        });
        self.head = Some(node);
        self
    }

    pub fn append_tail_imperative(&mut self, val: T) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node { val, next: None }));
    }

    pub fn append_tail_recursive(&mut self, val: T) {
        LinkedList::append_tail_recursive_helper(&mut self.head, val);
    }
    fn append_tail_recursive_helper(head: &mut Option<Box<Node<T>>>, val: T) {
        if let Some(head_box) = head {
            let next = &mut head_box.next;
            LinkedList::append_tail_recursive_helper(next, val);
        } else {
            *head = Some(Box::new(Node { val, next: None }));
        }
    }

    pub fn from_vec(v: Vec<T>) -> LinkedList<T> {
        let mut linked_list = LinkedList { head: None };
        for i in v.into_iter().rev() {
            linked_list.push_head(i);
        }

        linked_list
    }

    pub fn tail_value(&self) -> Option<&T> {
        if self.head.is_none() {
            return None;
        } else {
            let mut pre = self.head.as_ref().unwrap();
            while let Some(ref next) = pre.next {
                pre = next;
            }
            return Some(&pre.val);
        }
    }
}

impl<T: Eq + Display + Clone> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "None\n").unwrap();
        } else {
            let mut next = self.head.as_ref();
            while let Some(current) = next {
                write!(f, "{}->\n", current.val)?;
                next = current.next.as_ref();
            }
            write!(f, "None\n")?;
        }

        Ok(())
    }
}

impl<T: Eq + Display + Clone> Clone for LinkedList<T> {
    fn clone(&self) -> LinkedList<T> {
        let mut new_linked_list = LinkedList { head: None };
        let mut current = &self.head;
        while let Some(ref node) = current {
            new_linked_list.push_head(node.val.clone());
            current = &node.next;
        }

        new_linked_list
    }
}

impl<T: Eq + Display + Clone> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_value() {
        let mut linked_list = LinkedList::from_vec(vec![1, 2, 3, 4, 4]);
        assert_eq!(linked_list.tail_value(), Some(&4));
        linked_list.remove_tail();
        assert_eq!(linked_list.tail_value(), Some(&4));

        linked_list.remove_tail();
        assert_eq!(linked_list.tail_value(), Some(&3));
        linked_list.remove_tail();
        assert_eq!(linked_list.tail_value(), Some(&2));

        linked_list.remove_tail();
        assert_eq!(linked_list.tail_value(), Some(&1));

        linked_list.remove_tail();
        assert_eq!(linked_list.tail_value(), None);
    }

    #[test]
    fn test_display() {
        let linked_list = LinkedList::from_vec(vec![1, 2, 3, 4, 4]);
        println!("{}", linked_list);
    }

    #[test]
    fn test_remove_first_hit() {
        let mut linked_list = LinkedList::from_vec(vec![0, 1, 2, 2, 2, 3, 4]);
        linked_list.remove_first_hit(2);
        assert_eq!(linked_list.remove_head().unwrap(), 0);
        assert_eq!(linked_list.remove_head().unwrap(), 1);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
        assert_eq!(linked_list.remove_head().unwrap(), 3);
        assert_eq!(linked_list.remove_head().unwrap(), 4);
    }

    #[test]
    fn test_remove_all_hit() {
        let mut linked_list = LinkedList::from_vec(vec![0, 1, 1, 2]);
        linked_list.remove_all_hit(1);
        assert_eq!(linked_list.remove_head().unwrap(), 0);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
    }

    #[test]
    fn test_remove_tail() {
        let mut linked_list = LinkedList::from_vec(vec![0, 1, 2]);
        assert_eq!(linked_list.remove_tail().unwrap(), 2);
        assert_eq!(linked_list.remove_tail().unwrap(), 1);
        assert_eq!(linked_list.remove_tail().unwrap(), 0);
        assert_eq!(linked_list.remove_tail(), None);
    }

    #[test]
    fn test_append_tail_recursive() {
        let mut linked_list = LinkedList::from_vec(vec![0]);
        linked_list.append_tail_recursive(1);
        linked_list.append_tail_recursive(2);
        assert_eq!(linked_list.remove_head().unwrap(), 0);
        assert_eq!(linked_list.remove_head().unwrap(), 1);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
    }

    #[test]
    fn test_append_tail_imperative() {
        let mut linked_list = LinkedList::from_vec(vec![0]);
        linked_list.append_tail_imperative(1);
        linked_list.append_tail_imperative(2);
        assert_eq!(linked_list.remove_head().unwrap(), 0);
        assert_eq!(linked_list.remove_head().unwrap(), 1);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
    }

    #[test]
    fn test_reverse_imperative() {
        let mut linked_list = LinkedList::from_vec(vec![0, 1, 2, 3]);
        linked_list.reverse_imperative();
        assert_eq!(linked_list.remove_head().unwrap(), 3);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
        assert_eq!(linked_list.remove_head().unwrap(), 1);
    }

    #[test]
    fn test_reverse_recursive() {
        let mut linked_list = LinkedList::from_vec(vec![0, 1, 2, 3]);
        linked_list.reverse_recursive();
        assert_eq!(linked_list.remove_head().unwrap(), 3);
        assert_eq!(linked_list.remove_head().unwrap(), 2);
        assert_eq!(linked_list.remove_head().unwrap(), 1);
    }

    #[test]
    fn test_push_head() {
        let mut linked_list = LinkedList::from_vec(vec![0]);
        linked_list.push_head(1).push_head(2);

        assert_eq!(linked_list.remove_head().unwrap(), 2);
        assert_eq!(linked_list.remove_head().unwrap(), 1);
        assert_eq!(linked_list.remove_head().unwrap(), 0);
    }

    #[test]
    fn test_from_vec() {
        let mut linked_list = LinkedList::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(1, linked_list.remove_head().unwrap());
        assert_eq!(2, linked_list.remove_head().unwrap());
        assert_eq!(3, linked_list.remove_head().unwrap());
        assert_eq!(4, linked_list.remove_head().unwrap());
        assert_eq!(5, linked_list.remove_head().unwrap());
        assert_eq!(None, linked_list.remove_head());
    }
}