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

        // 1. ref mut next: is mutable borrow, but if next is moved. comparing this:
        // while let Some(next) = nodex_ref.next{} here is partial move.
        while let Some(ref mut next) = node_ref.next {
            if next.val == 2 {
                // 2. take() is very important, which move the Value but keep the Option as None
                node_ref.next = next.next.take();
            } else {
                // 3. below compile failed, because next is partially mutable borrow from node_ref, so node_ref itself can't changed.
                // node_ref = next.deref_mut();
                node_ref = node_ref.next.as_mut().unwrap();
            }
        }

        // 4. no partial move. which means that the next is still valid.
        // the node.next.take() doesn't equals to `let a = node.next; let b = a.take();`
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
