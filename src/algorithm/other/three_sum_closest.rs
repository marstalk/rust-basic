/**
Given an integer array nums of length n and an integer target,
find three integers in nums such that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.



Example 1:

Input: nums = [-1,2,1,-4], target = 1
Output: 2
Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
Example 2:

Input: nums = [0,0,0], target = 1
Output: 0
Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).


Constraints:

3 <= nums.length <= 500
-1000 <= nums[i] <= 1000
-104 <= target <= 104
 */

pub struct Solution {}
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // get the earliest sum.
        fn helper(sorted_nums: &Vec<i32>, target: i32, mut start: usize, mut end: usize) -> i32 {
            // let mut res = i32::MAX; // this will cause res-target overflow when target is negative.
            let mut res = sorted_nums[start] + sorted_nums[end];
            while start < end {
                let sum = sorted_nums[start] + sorted_nums[end];

                // any sum with smaller absolute value is better.
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }
                if sum < target {
                    start += 1;
                } else if sum > target {
                    end -= 1;
                } else {
                    return sorted_nums[start] + sorted_nums[end];
                }
            }

            res
        }

        nums.sort();
        println!("{:?}", nums);

        // don't use let mut res = i32::MAX; // this will cause res-target overflow when target is negative.
        let mut res = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let sub_target = target - nums[i];
            let rtn = helper(&nums, sub_target, i + 1, nums.len() - 1);
            println!(
                "target {} for {}, sub_target {}, rtn {}",
                target, nums[i], sub_target, rtn
            );
            if (res - target).abs() > (rtn + nums[i] - target).abs() {
                res = rtn + nums[i];
            }
        }
        res
    }

    /**
     * optimized three_sum_closest() solution above.
     */
    pub fn three_sum_closest_optimized(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        //TODO
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(Solution::three_sum_closest(vec![0, 1, 2], 0), 3);
        assert_eq!(Solution::three_sum_closest(vec![2, 3, 8, 9, 10], 16), 15);
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 1], 0), 3);
    }
}
