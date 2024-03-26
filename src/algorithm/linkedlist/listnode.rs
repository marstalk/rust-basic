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
    pub fn remove_first(&mut self, val: i32) {
        // TODO implement remove first hit method
    }

    pub fn from_vec(v: Vec<i32>) -> LinkedList {
        let mut linked_list = LinkedList { head: None };
        for i in v.into_iter().rev() {
            linked_list.push_head(i);
        }

        linked_list
    }
}

/**
 * reverse linked list
 */
pub fn reverse_linked_list(head: Option<LinkedList>) -> Option<LinkedList> {
    // TODO implement the reverse linked list method
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let mut linked_list = LinkedList::from_vec(vec![1, 2, 3, 4, 5]);
        println!("{:?}", linked_list);
        assert_eq!(1, linked_list.remove_head().unwrap());
        assert_eq!(2, linked_list.remove_head().unwrap());
        assert_eq!(3, linked_list.remove_head().unwrap());
        assert_eq!(4, linked_list.remove_head().unwrap());
        assert_eq!(5, linked_list.remove_head().unwrap());
        assert_eq!(None, linked_list.remove_head());
    }
}
