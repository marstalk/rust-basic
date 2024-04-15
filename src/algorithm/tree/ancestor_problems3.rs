use crate::algorithm::bst::bst::{Bst, Node};

/**
 * p and q must in the binary search tree.
 *
 */
pub fn lowest_common_ancestor_bst(bst: &Bst, p: i32, q: i32) -> Option<i32> {
    fn helper(node: &Option<Box<Node>>, min: i32, max: i32) -> Option<i32> {
        // 1. base
        if let Some(node) = node {
            if node.val < min {
                // 2. if min is less that left, then go to left to find LCA
                return helper(&node.right, min, max);
            } else if node.val > max {
                // 2.1 if max is greater than right, then go to right to find LCA
                return helper(&node.left, min, max);
            } else {
                // 2.2 then this node is the LCA
                return Some(node.val);
            }
        } else {
            return None;
        }
    }

    let min = p.min(q);
    let max = p.max(q);
    helper(&bst.root, min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor_bst() {
        assert_eq!(
            lowest_common_ancestor_bst(&Bst::from_vector(vec![3, 5, 1, 6, 2, 0, 8, 9, 7]), 6, 7),
            Some(6)
        )
    }
}
