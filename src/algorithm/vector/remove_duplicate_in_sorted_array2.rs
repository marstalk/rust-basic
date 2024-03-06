/***
Given an integer array nums sorted in non-decreasing order,
remove some duplicates in-place such that each unique element appears at [most twice].

The relative order of the elements should be kept the same.

Since it is impossible to change the length of the array in some languages,
you must instead have the result be placed in the first part of the array nums.

More formally, if there are k elements after removing the duplicates,
then the first k elements of nums should hold the final result.

It does not matter what you leave beyond the first k elements.
Return k after placing the final result in the first k slots of nums.
Do not allocate extra space for another array.
You must do this by modifying the input array in-place with O(1) extra memory.
 */

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = -1i32;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            //
            if map.contains_key(&nums[i]) && map.get(&nums[i]).unwrap() >= &2 {
                // skip it.
            } else {
                //if nums[i] is not in the map, keep it.
                map.insert(nums[i], map.get(&nums[i]).unwrap_or(&0) + 1);
                idx += 1;
                nums[idx as usize] = nums[i];
            }
        }
        idx + 1
    }

    // because it's  non-decreasing order, so we can use double pointer.
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut idx = 1usize;
        for i in 1..nums.len() {
            if nums[i] != nums[idx] {
                idx += 1;
                nums[idx] = nums[i];
            } else if nums[idx] != nums[idx - 1] && idx < i {
                // make sure idx < i, to handle the case of [1,2,3] when idx=1,i=1
                idx += 1;
                nums[idx] = nums[i];
            }
        }
        idx as i32 + 1
    }

    // double pointer fast and slow
    // slow is the index which is not handled yet.
    pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut slow = 2;
        let mut fast = 2;
        while fast < nums.len() {
            if nums[fast] != nums[slow - 2] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dupliates3() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates3(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums[0..k as usize], vec![1, 1, 2, 2, 3]);

        let mut nums = vec![1, 2, 2];
        let k = Solution::remove_duplicates3(&mut nums);
        assert_eq!(k, 3);
        assert_eq!(nums[0..k as usize], vec![1, 2, 2]);

        let mut nums = vec![1, 2, 3];
        let k = Solution::remove_duplicates3(&mut nums);
        assert_eq!(k, 3);
        assert_eq!(nums[0..k as usize], vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_dupliates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums[0..k as usize], vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_remove_dupliates2() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates2(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums[0..k as usize], vec![1, 1, 2, 2, 3]);

        let mut nums = vec![1, 2, 2];
        let k = Solution::remove_duplicates2(&mut nums);
        assert_eq!(k, 3);
        assert_eq!(nums[0..k as usize], vec![1, 2, 2]);

        let mut nums = vec![1, 2, 3];
        let k = Solution::remove_duplicates2(&mut nums);
        assert_eq!(k, 3);
        assert_eq!(nums[0..k as usize], vec![1, 2, 3]);
    }
}
