pub struct Node {
    pub val: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct Bst {
    pub root: Option<Box<Node>>,
}

impl Bst {
    pub fn from_vector(vec: Vec<i32>) -> Bst {
        let mut bst = Bst { root: None };
        for val in vec {
            bst.append_recursive(val);
        }
        bst
    }

    pub fn append_recursive(&mut self, val: i32) {
        fn append_to_node(node: &mut Box<Node>, val: i32) {
            if val <= node.as_ref().val {
                match node.as_mut().left {
                    Some(ref mut left_node) => {
                        append_to_node(left_node, val);
                    }
                    None => {
                        node.left = Some(Box::new(Node {
                            val,
                            left: None,
                            right: None,
                        }));
                    }
                }
            } else {
                match node.as_mut().right {
                    Some(ref mut right_node) => {
                        append_to_node(right_node, val);
                    }
                    None => {
                        node.right = Some(Box::new(Node {
                            val,
                            left: None,
                            right: None,
                        }));
                    }
                }
            }
        }
        match self.root.as_mut() {
            Some(ref mut root) => {
                append_to_node(root, val);
            }
            None => {
                self.root = Some(Box::new(Node {
                    val,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    /**
     * travese the bst tree in mid order so that we can get the result in ascending order.
     */
    pub fn to_string_asc(&self) -> String {
        let mut res = String::from("");
        fn traverse(node: &Option<Box<Node>>, res: &mut String) {
            if let Some(node) = node {
                traverse(&node.left, res);
                res.push_str(node.val.to_string().as_str());
                res.push(',');
                traverse(&node.right, res);
            }
        }
        traverse(&self.root, &mut res);
        res
    }

    pub fn to_string_desc(&self) -> String {
        let mut res = String::from("");
        fn traverse(node: &Option<Box<Node>>, res: &mut String) {
            if let Some(node) = node {
                traverse(&node.right, res);
                res.push_str(node.val.to_string().as_str());
                res.push(',');
                traverse(&node.left, res);
            }
        }
        traverse(&self.root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_desc() {
        assert_eq!(
            Bst::from_vector(vec![5, 2, 3, 54, 8, 3, 1, 111, 435, 0]).to_string_desc(),
            "435,111,54,8,5,3,3,2,1,0,"
        );
    }

    #[test]
    fn test_mid_order_traverse() {
        let bst = Bst::from_vector(vec![3, 5, 1, 6, 2, 0, 8, 9, 7, 4]);
        assert_eq!(bst.to_string_asc(), "0,1,2,3,4,5,6,7,8,9,");
    }

    #[test]
    fn test_to_string_asc() {
        let bst = Bst::from_vector(vec![3, 5, 1, 6, 2, 0, 8, 9, 7]);
        assert_eq!(bst.root.as_ref().unwrap().val, 3);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().val, 1);
    }
}
