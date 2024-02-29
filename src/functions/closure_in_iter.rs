/**
 * iter.find, any, all, position, for_each.
 */

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter_any() {
        let nums = vec![1, 2, 3, 4];
        // all: if all item meet the need, return true; else false.
        assert_eq!(nums.iter().all(|x| x > &0), true);
        // any: any one meet the need, return true; else false.
        assert_eq!(nums.iter().any(|x| x > &5), false);
        assert_eq!(nums[0], 1);
    }

    #[test]
    fn test_iter_moved() {
        let nums = vec![2, 3, 4, 5];
        // for_each
        nums.into_iter().for_each(|item| println!("{}", item));
        // nums become invalid.
        //println!("{:?}", nums);
    }

    #[test]
    fn test_iter_find() {
        let nums = vec![1, 2, 3];
        // find the first item which match the condition.
        let rtn = nums.iter().find(|item| **item == 3);
        assert_eq!(rtn, Some(&3));
    }

    #[test]
    fn test_iter_position() {
        let nums = vec![1, 2, 3];
        // find the position.
        assert_eq!(nums.iter().position(|x| x == &3), Some(2));
    }
}
