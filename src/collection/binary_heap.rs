#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    #[test]
    fn test_bianry_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(4);
        heap.push(2);
        heap.push(88);
        heap.push(5);

        assert_eq!(heap.pop(), Some(88));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_bianry_heap2() {
        let mut heap = BinaryHeap::new();
        heap.push((3, 0));
        heap.push((1, 1));
        heap.push((9, 2));
        heap.push((6, 3));

        assert_eq!(heap.pop(), Some((9, 2)));
        assert_eq!(heap.pop(), Some((6, 3)));
        assert_eq!(heap.pop(), Some((3, 0)));
        assert_eq!(heap.pop(), Some((1, 1)));
        assert_eq!(heap.pop(), None);
    }
}
