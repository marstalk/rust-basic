use crate::algorithm::tree::binary_tree::{BTree, Node};

/**
 * p or q may not in the tree && all the nodes are unique.
 * post-order iteration.
 */
pub fn lowest_common_ancestor2(btree: &BTree, p: i32, q: i32) -> Option<i32> {
    fn helper(node: &Option<Box<Node>>, p: i32, q: i32) -> (bool, bool, Option<i32>) {
        //base
        if node.is_none() {
            return (false, false, None);
        }

        let (left_hit_p, left_hit_q, left_rtn) = helper(&node.as_ref().unwrap().left, p, q);
        let (right_hit_p, right_hit_q, right_rtn) = helper(&node.as_ref().unwrap().right, p, q);

        // if left_rtn not none && right_rtn not none, then node the the LCA
        if left_rtn.is_some() && right_rtn.is_some() {
            return (true, true, Some(node.as_ref().unwrap().val));
        }

        // if current node equals to p, then return p
        let hit_p = left_hit_p || right_hit_p;
        let hit_q = left_hit_q || right_hit_q;
        if node.as_ref().unwrap().val == p {
            return (true, hit_q, Some(node.as_ref().unwrap().val));
        }

        // if current node equals to q, then return q
        if node.as_ref().unwrap().val == q {
            return (hit_p, true, Some(node.as_ref().unwrap().val));
        }

        // other case.
        if left_rtn.is_some() {
            return (hit_p, hit_q, left_rtn);
        } else if right_rtn.is_some() {
            return (hit_p, hit_q, right_rtn);
        } else {
            return (hit_p, hit_q, None);
        }
    }

    let (hit_p, hit_q, rtn) = helper(&btree.root, p, q);

    // only when the two nodes are in the tree then the rtn make sense.
    if hit_p && hit_q && rtn.is_some() {
        return rtn;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::tree::binary_tree::BTree;

    #[test]
    fn test_lowest_common_ancestor2() {
        let tree = BTree::from_vec(vec![3, 5, 1, 6, 2, 0, 8, 9, 7]);
        assert_eq!(lowest_common_ancestor2(&tree, 6, 7), Some(6));
        assert_eq!(lowest_common_ancestor2(&tree, 0, 7), Some(3));
        assert_eq!(lowest_common_ancestor2(&tree, 0, 77), None);
    }
}
