/***
https://leetcode.cn/problems/remove-duplicates-from-sorted-array/submissions/504623758/
Given an integer array nums sorted in non-decreasing order,
remove the duplicates in-place such that each unique element appears only once.
The relative order of the elements should be kept the same.
Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k,
to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially.
The remaining elements of nums are not important as well as the size of nums.

Return k.
 */

pub struct Solution();

impl Solution {
    /***
     * because sorted, then we can use the fast and slow pointer.
     * fast pointer to loop from 0 to n-1,
     * while slow pointer moves one step forward only when value needed to change.
     */
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut fast = 0;
        let mut slow = 0;
        while fast < nums.len() {
            if fast == slow {
                fast += 1;
                continue;
            } else if nums[fast] == nums[slow] {
                fast += 1;
                continue;
            } else if slow + 1 == fast {
                slow += 1;
                fast += 1;
                continue;
            }
            slow += 1;
            nums[slow] = nums[fast];
            fast += 1;
        }

        (slow + 1) as i32
    }

    // can't do by iter()
    // pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32{
    //     let mut idx = 0;
    //     for val in nums.iter_mut(){
    //         if *val != nums[idx]{
    //             idx += 1;
    //             nums[idx] = *val;
    //         }
    //     }

    //     (idx + 1) as i32
    // }

    /***
     * loop and swap with index
     */
    pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != nums[idx] {
                idx += 1;
                nums[idx] = nums[i];
            }
        }

        (idx + 1) as i32
    }

    // double pointer.
    pub fn remove_duplicates4(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut slow = 1usize;
        let mut fast = 1usize;
        while fast < nums.len() {
            if nums[fast] != nums[slow - 1] {
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
    fn test_remove_duplicates4() {
        let mut nums = vec![1, 1, 2];
        let k = Solution::remove_duplicates4(&mut nums);
        assert_eq!(k, 2);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = Solution::remove_duplicates4(&mut nums);
        assert_eq!(k, 5);
    }

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 2);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
    }

    #[test]
    fn test_remove_duplicates3() {
        let mut nums = vec![1, 1, 2];
        let k = Solution::remove_duplicates3(&mut nums);
        assert_eq!(k, 2);
    }
}
