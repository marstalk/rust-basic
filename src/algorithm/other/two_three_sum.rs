pub struct Solution {}

/**
 * 1. two sum
 * 2. three sum
 * 3. four sum
 * 4. n sum TODO
 */
impl Solution {
    pub fn all_two_sum(mut nums: Vec<i32>, target: i32) -> Option<Vec<Vec<i32>>> {
        let mut res = Vec::new();

        // 1. sort
        nums.sort();

        // 2. two pointers
        let mut start = 0usize;
        let mut end = nums.len() - 1;

        while start < end {
            let left_val = nums[start];
            let right_val = nums[end];
            let sum = left_val + right_val;
            if sum > target {
                // 3. sum is too big, move end to smaller index until nums[end] != right_val
                while start < end && nums[end] == right_val {
                    end -= 1;
                }
            } else if sum < target {
                // 4. sum is too small, move start to bigger index until nums[start] != left_val
                while start < end && nums[start] == left_val {
                    start += 1;
                }
            } else {
                // 5. hit valid combination, and move forward to avoid dulication
                res.push(vec![left_val, right_val]);

                while start < end && nums[end] == right_val {
                    end -= 1;
                }
                while start < end && nums[start] == left_val {
                    start += 1;
                }
            }
        }

        if res.len() == 0 {
            None
        } else {
            Some(res)
        }
    }

    pub fn all_three_sum(mut nums: Vec<i32>, target: i32) -> Option<Vec<Vec<i32>>> {
        let mut res = Vec::new();

        // 1. sort in ascending order.
        nums.sort();

        // 2. iterator over the nums
        let mut i = 0;
        while i < nums.len() {
            let num = nums[i];
            let two_target = target - num;

            // 2.1 find all tow_sums
            let rtn =
                Solution::all_two_sum_with_sorted_nums(&nums, two_target, i + 1, nums.len() - 1);
            for item in rtn.into_iter() {
                res.push(vec![num, item[0], item[1]]);
            }

            // 2.2 avoid duplication
            while i < nums.len() - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }

            // 2.3 next different num
            i += 1;
        }

        // 3. return
        if res.len() == 0 {
            None
        } else {
            Some(res)
        }
    }

    fn all_two_sum_with_sorted_nums(
        nums: &Vec<i32>,
        target: i32,
        mut start: usize,
        mut end: usize,
    ) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        while start < end {
            let left_val = nums[start];
            let right_val = nums[end];
            let sum = left_val + right_val;
            if sum > target {
                while start < end && nums[end] == right_val {
                    end -= 1;
                }
            } else if sum < target {
                while start < end && nums[start] == left_val {
                    start += 1;
                }
            } else {
                res.push(vec![left_val, right_val]);
                while start < end && nums[end] == right_val {
                    end -= 1;
                }
                while start < end && nums[start] == left_val {
                    start += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_three_sum() {
        assert!(Solution::all_three_sum(vec![1, 2, 3, 4, 5], 90).is_none());
        assert_eq!(
            Solution::all_three_sum(vec![1, 2, 3, 4, 5], 7).unwrap(),
            vec![vec![1, 2, 4]]
        );
        assert_eq!(
            Solution::all_three_sum(vec![1, 2, 3, 4, 5, 6, 7], 9).unwrap(),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }

    #[test]
    fn test_all_two_sum() {
        assert!(Solution::all_two_sum(vec![1, 2, 3, 4, 5], 90).is_none());
        assert_eq!(
            Solution::all_two_sum(vec![1, 2, 3, 4, 5], 7).unwrap(),
            vec![vec![2, 5], vec![3, 4]]
        );
        assert_eq!(
            Solution::all_two_sum(vec![1, 2, 3, 4, 5, 6, 7], 8).unwrap(),
            vec![vec![1, 7], vec![2, 6], vec![3, 5]]
        );
    }
}