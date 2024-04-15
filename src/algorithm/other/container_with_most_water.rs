/**
You are given an integer array height of length n.
There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.
Return the maximum amount of water a container can store.
Notice that you may not slant the container.



Example 1:
Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
In this case, the max area of water (blue section) the container can contain is 49.

Example 2:
Input: height = [1,1]
Output: 1

Constraints:
n == height.length
2 <= n <= 105
0 <= height[i] <= 104
 */
pub struct Solution {}
impl Solution {
    /**
     * method 1: use nested loop to culculate all combinations and find the maximum.
     * time complexity: O(n^2)
     */
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..height.len() {
            for j in i + 1..height.len() {
                let area = (j - i) as i32 * height[i].min(height[j]);
                res = res.max(area);
            }
        }
        res
    }

    /**
     * method 2: use two pointers to find the maximum area. always move the smaller pointer.
     * time complexity: O(n)
     * greedy algorithm
     */
    pub fn max_area_two_pointers(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut res = 0;
        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);
            res = res.max(area);
            if height[left] < height[right] {
                left += 1;
            } else {
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
    fn test_max_area_two_pointers() {
        assert_eq!(
            Solution::max_area_two_pointers(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]),
            49
        );
        assert_eq!(Solution::max_area_two_pointers(vec![1, 1]), 1);
    }

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
