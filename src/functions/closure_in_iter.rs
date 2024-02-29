// argument is closure and vector, then apply the function to every item

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter_closure() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(nums.iter().all(|x| x > &0), true);
        assert_eq!(nums.iter().any(|x| x > &5), false);
        assert_eq!(nums[0], 1);
    }

    #[test]
    fn test_iter_closure2() {
        let nums = vec![2, 3, 4, 5];
        nums.into_iter().for_each(|item| println!("{}", item));
        // nums become invalid.
        //println!("{:?}", nums);
    }

    #[test]
    fn test_iter_closure3() {
        let nums = vec![1, 2, 3];
        let rtn = nums.iter().find(|item| **item == 3);
        assert_eq!(rtn, Some(&3));
    }
}
