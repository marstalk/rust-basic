#![allow(dead_code)]

use std::collections::LinkedList;

pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
pub struct BTree {
    pub root: Option<Box<Node>>,
}
impl BTree {
    pub fn from_vec(vec: Vec<i32>) -> BTree {
        if vec.is_empty() {
            return BTree { root: None };
        }

        let mut root = Node {
            val: vec[0],
            left: None,
            right: None,
        };
        let mut list: LinkedList<&mut Node> = LinkedList::new();
        list.push_back(&mut root);
        let mut idx = 0;
        loop {
            idx += 1;
            if idx >= vec.len() {
                break;
            }
            let node = list.pop_front().unwrap();
            node.left = Some(Box::new(Node {
                val: vec[idx],
                left: None,
                right: None,
            }));
            list.push_back(node.left.as_mut().unwrap());

            idx += 1;
            if idx >= vec.len() {
                break;
            }
            node.right = Some(Box::new(Node {
                val: vec[idx],
                left: None,
                right: None,
            }));
            list.push_back(node.right.as_mut().unwrap());
        }

        BTree {
            root: Some(Box::new(root)),
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::from("");

        let mut list = LinkedList::new();
        list.push_back(self.root.as_ref());

        while let Some(node) = list.pop_front() {
            match node {
                Some(node) => {
                    res += &node.val.to_string();
                    list.push_back(node.left.as_ref());
                    list.push_back(node.right.as_ref());
                }
                None => {}
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let btree = BTree::from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(btree.to_string(), "1234567");
    }
}
