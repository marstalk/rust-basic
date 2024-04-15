use crate::algorithm::tree::binary_tree::{BTree, Node};

/**
 * Attention: p & q must be in the tree && all the nodes are unique.
 * pre-order iteration.
 */
pub fn lowest_common_ancestor(btree: &BTree, p: i32, q: i32) -> i32 {
    fn helper(node: &Option<Box<Node>>, p: i32, q: i32) -> Option<i32> {
        if node.is_none() {
            return None;
        }

        // 1. if root equals to q or p, return root.
        if node.as_ref().unwrap().val == p || node.as_ref().unwrap().val == q {
            return Some(node.as_ref().unwrap().val);
        }
        // 2. if left_rtn is n
        let left_rtn = helper(&node.as_ref().unwrap().left, p, q);
        let right_rtn = helper(&node.as_ref().unwrap().right, p, q);
        if left_rtn.is_none() && right_rtn.is_none() {
            return None;
        } else if left_rtn.is_some() && right_rtn.is_some() {
            return Some(node.as_ref().unwrap().val);
        } else if left_rtn.is_some() {
            return left_rtn;
        } else {
            return right_rtn;
        }
    }

    match helper(&btree.root, p, q) {
        Some(val) => val,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::tree::binary_tree::BTree;

    #[test]
    fn test_lowest_common_ancestor() {
        let tree = BTree::from_vec(vec![3, 5, 1, 6, 2, 0, 8, 9, 7]);
        assert_eq!(lowest_common_ancestor(&tree, 6, 7), 6);
        assert_eq!(lowest_common_ancestor(&tree, 0, 7), 3);
        assert_eq!(lowest_common_ancestor(&tree, 9, 2), 5);
    }
}
