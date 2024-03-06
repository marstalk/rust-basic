/**
Given an integer array nums,
move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array.
 */

pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idx = -1i32;
        // remove all non-zero to the left part.
        for i in 0..nums.len() {
            if nums[i] != 0 {
                idx += 1;
                nums[idx as usize] = nums[i];
            }
        }
        // set the left element to the zero.
        for i in (idx + 1) as usize..nums.len() {
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
