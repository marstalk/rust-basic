#![allow(unused_variables)]

#[derive(Debug)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
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
        mut current: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    ) {
        if let Some(mut current) = current.take() {
            let next_node = current.next.take();
            current.next = prev;
            self.reverse_recursive_helper(next_node, Some(current));
        } else {
            // if current node is None which means it's the end of linked list, so set prev (the last node) as the new head.
            self.head = prev;
        }
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

    pub fn push_head(&mut self, val: i32) {
        let node = Box::new(Node {
            val: val,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn remove_head(&mut self) -> Option<i32> {
        let head_node = self.head.take();
        match head_node {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }

    /**
     * remove target element if exist.
     */
    pub fn remove_first_hit(&mut self, val: i32) {
        //TODO
    }

    pub fn from_vec(v: Vec<i32>) -> LinkedList {
        let mut linked_list = LinkedList { head: None };
        for i in v.into_iter().rev() {
            linked_list.push_head(i);
        }

        linked_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
