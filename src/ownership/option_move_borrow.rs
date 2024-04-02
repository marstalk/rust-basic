#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        let third = Some(Box::new(Node { val: 3, next: None }));
        let next = Some(Box::new(Node {
            val: 2,
            next: third,
        }));
        let mut node = Node { val: 1, next: next };

        let mut node_ref = &mut node;

        while let Some(ref mut next) = node_ref.next {
            if next.val == 2 {
                node_ref.next = next.next.take();
            } else {
                // below compile failed, because next is partially mutable borrow from node_ref, wo node_ref itself can't changed.
                // node_ref = next.deref_mut();
                node_ref = node_ref.next.as_mut().unwrap();
            }
        }

        // no partial move. which means that the next is still valid.
        let third = node.next.take();
        assert_eq!(third.is_some(), true);
        assert_eq!(third.unwrap().val, 3);

        assert_eq!(node.next.is_none(), true);
        assert_eq!(node.val, 1);

        // partial moved, and node.next is no longer valid.
        let _a = node.next;
        // let _b = node.next;
    }
}
