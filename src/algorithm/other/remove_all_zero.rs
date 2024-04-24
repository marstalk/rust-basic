/**
 * remove all zero from the end of the array in place.
 */
pub struct Solution {}
impl Solution {
    pub fn remove_all_zero(nums: &mut Vec<i32>) {
        let mut fast = 0;
        let mut slow = 0;

        while fast < nums.len() {
            if nums[fast] != 0 {
                nums[slow] = nums[fast];
                nums[fast] = 0;
                slow += 1;
            }
            fast += 1;
        }
    }

    pub fn remove_all_zero2(nums: &mut Vec<i32>) {
        let mut fast = 0;
        let mut slow = 0;
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        while slow < nums.len() {
            nums[slow] = 0;
            slow += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_all_zero2() {
        let mut nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        Solution::remove_all_zero2(&mut nums);
        assert_eq!(nums, vec![1, 2, 1, 1, 3, 2, 1, 2, 1, 0, 0, 0]);
    }

    #[test]
    fn test_remove_all_zero() {
        let mut nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        Solution::remove_all_zero(&mut nums);
        assert_eq!(nums, vec![1, 2, 1, 1, 3, 2, 1, 2, 1, 0, 0, 0]);
    }
}
