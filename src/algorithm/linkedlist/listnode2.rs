#[allow(dead_code)]
/**
 *
 */
#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn append_tail(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            Some(mut head_rc) => {
                let next = head_rc.next.take();
                let next = ListNode::append_tail(next, val);
                head_rc.next = next;
                Some(head_rc)
            }
            None => Some(Box::new(ListNode { val, next: None })),
        }
    }

    pub fn append_head(self, val: i32) -> ListNode {
        ListNode {
            val,
            next: Some(Box::new(self)),
        }
    }

    pub fn append_tail2(&mut self, val: i32) {
        let next = self.next.take();
        match next {
            Some(mut next) => {
                next.append_tail2(val);
                self.next = Some(next);
            }
            None => {
                self.next = Some(Box::new(ListNode { val, next: None }));
            }
        }
    }

    pub fn remove_head(&mut self) -> Option<Box<ListNode>> {
        self.next.take()
    }

    pub fn reverse(mut self) -> ListNode {
        let mut vec = Vec::new();
        let mut next = self.next.take();
        vec.push(self);

        while let Some(mut next_item) = next {
            next = next_item.next.take();
            vec.push(*next_item);
        }

        vec.reverse();

        ListNode { val: 3, next: None }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reverse() {
        let mut head = ListNode { val: 0, next: None };
        head.append_tail2(1);
        head.append_tail2(2);
        let head = head.append_head(3);

        let mut new_head = head.reverse();
        assert_eq!(new_head.remove_head().unwrap().val, 3);
        assert_eq!(new_head.remove_head().unwrap().val, 2);
        assert_eq!(new_head.remove_head().unwrap().val, 1);
        assert_eq!(new_head.remove_head().unwrap().val, 0);
    }

    #[test]
    fn test_append_tail() {
        let head = ListNode::append_tail(None, 1);
        let head = ListNode::append_tail(head, 2);
        let mut head = head.unwrap().append_head(0);
        head.append_tail2(3);

        let head = head.remove_head();
        println!("{:?}", head);
    }
}
