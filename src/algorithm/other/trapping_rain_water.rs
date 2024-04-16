/**
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.


Example 1:
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1].
In this case, 6 units of rain water (blue section) are being trapped.

Example 2:
Input: height = [4,2,0,3,2,5]
Output: 9


Constraints:
n == height.length
1 <= n <= 2 * 104
0 <= height[i] <= 105
 */

pub struct Solution {}
impl Solution {
    /**
     * brute force, for every bar, find it's left higest and right higest, then the
     * trap would be min(left_higest, right_higest) - height[i];
     * time complexity: O(n^2)
     */
    pub fn trap_brute_force(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..heights.len() - 1 {
            // 1. find the highest in the left.
            let mut left_higest = 0;
            for left in 0..=i {
                left_higest = left_higest.max(heights[left]);
            }

            // 2. find the highest in the right.
            // excluding current bar: for right in i+1..heights.len()
            let mut right_highest = 0;
            for right in i..heights.len() {
                right_highest = right_highest.max(heights[right]);
            }

            // 3. add to the res if (left_higest.min(right_highest)) - heights[i] is greater than 0.
            // if left_higest and right_highest include the current bar, then the minus operationi will not be negative.
            res += (left_higest.min(right_highest)) - heights[i];
        }
        res
    }

    /**
     * use memory to help.
     * 1. left_highests[i] represents the left highest bar for the i.
     * 2. right_highests[i] represents the right highest bar for the i.
     */
    pub fn trap_memo(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut res = 0;

        // 1. loop from left to right. including current.
        let mut left_max = vec![0; n];
        left_max[0] = heights[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(heights[i]);
        }

        // 2. loop from right to left. including current.
        let mut right_max = vec![0; n];
        right_max[n - 1] = heights[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = right_max[i + 1].max(heights[i]);
        }

        // iterate all the bar.
        for i in 1..n - 1 {
            res += left_max[i].min(right_max[i]) - heights[i];
        }

        res
    }

    /**
     * two pointer.
     * left = 0, right = n;
     * use the left_max represents the max bar from [0..left]
     * and the right_max represents the max bar from [right..n]
     *
     * and for bar of i, equals to min(left_max, right_max) - height[i];
     *
     * ATTENTION, this method is quiet different from the the trap_memo.
     */
    pub fn trap_double_pointer(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0usize;
        let mut right = heights.len() - 1;
        let mut left_max = heights[left];
        let mut right_max = heights[right];

        while left < right {
            left_max = left_max.max(heights[left]);
            right_max = right_max.max(heights[right]);

            if left_max < right_max {
                // as long as the left_max < right_max, this bar could trap water of left_max - height[i];
                res += left_max - heights[left];
                left += 1;
            } else {
                res += right_max - heights[right];
                right -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_double_pointer() {
        assert_eq!(
            Solution::trap_double_pointer(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(Solution::trap_double_pointer(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_trap_memo() {
        assert_eq!(
            Solution::trap_memo(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(Solution::trap_memo(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_trap() {
        assert_eq!(
            Solution::trap_brute_force(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(Solution::trap_brute_force(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
