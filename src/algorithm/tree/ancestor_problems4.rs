use crate::algorithm::tree::binary_tree::{BTree, Node};

/**
 * given a list of values, find the LCA, all the values are in the binary tree.
 *
 */
pub fn lowest_common_ancestor(tree: &BTree, nums: Vec<i32>) -> Option<i32> {
    fn helper(node: &Option<Box<Node>>, nums: &Vec<i32>) -> Option<i32> {
        if let Some(node) = node {
            if nums.contains(&node.val) {
                return Some(node.val);
            }
            let left_rtn = helper(&node.left, nums);
            let right_rtn = helper(&node.right, nums);
            return match (left_rtn, right_rtn) {
                (Some(_), Some(_)) => Some(node.val),
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                _ => None,
            };
        }
        None
    }

    helper(&tree.root, &nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let tree = BTree::from_vec(vec![3, 5, 1, 6, 2, 0, 8, 9, 7]);
        assert_eq!(lowest_common_ancestor(&tree, vec![9, 7, 2]), Some(5));
        assert_eq!(lowest_common_ancestor(&tree, vec![9, 7, 6]), Some(6));
        assert_eq!(lowest_common_ancestor(&tree, vec![9, 7, 2, 0]), Some(3));
    }
}
