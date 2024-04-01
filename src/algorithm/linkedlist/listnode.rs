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
        mut current: Option<Box<Node>>,
        prev: Option<Box<Node>>,
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

    pub fn remove_tail(&mut self) -> Option<i32> {
        if let Some(mut current) = &mut self.head {
            // Check if the list has only one element
            if current.next.is_none() {
                return self.head.take().map(|node| node.val);
            }

            // Traverse until the second-to-last node
            while let Some(ref mut next_node) = current.next {
                if next_node.next.is_none() {
                    // Remove the last node
                    let removed_node = current.next.take();
                    return removed_node.map(|node| node.val);
                }
                current = next_node;
            }
        }
        None // If the list is empty
    }

    /**
     * remove target element if exist.
     */
    pub fn remove_first_hit(&mut self, val: i32) {
        self.head = LinkedList::remove_elements(self.head.take(), val, false);
    }

    pub fn remove_hit(&mut self, val: i32) {
        let head = self.head.take();
        self.head = LinkedList::remove_elements(head, val, true);
    }

    fn remove_elements(head: Option<Box<Node>>, val: i32, all: bool) -> Option<Box<Node>> {
        let mut dummy = Node { val: 0, next: head };
        let mut pre = &mut dummy;
        while let Some(ref mut node) = pre.next {
            if node.val == val {
                pre.next = node.next.take();
                if !all {
                    break;
                }
            } else {
                pre = pre.next.as_mut().unwrap();
            }
        }
        dummy.next
    }

    pub fn push_head(&mut self, val: i32) {
        let node = Box::new(Node {
            val: val,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn append_tail_imperative(&mut self, val: i32) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node { val, next: None }));
    }

    pub fn append_tail_recursive(&mut self, val: i32) {
        LinkedList::append_tail_recursive_helper(&mut self.head, val);
    }
    fn append_tail_recursive_helper(head: &mut Option<Box<Node>>, val: i32) {
        if let Some(head_box) = head {
            let next = &mut head_box.next;
            LinkedList::append_tail_recursive_helper(next, val);
        } else {
            *head = Some(Box::new(Node { val, next: None }));
        }
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
        linked_list.push_head(1);
        linked_list.push_head(2);

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
