/**
 * Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
Return k.
 */

pub struct Solution();
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = -1;
        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            }
            idx += 1;
            nums[idx as usize] = val;
        }

        idx + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        //nums = [3,2,2,3], val = 3
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);

        //nums = [0,1,2,2,3,0,4,2], val = 2
        assert_eq!(
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }
}
