/**
* 给定一个非负整数 c ，你要判断是否存在两个整数 a 和 b，使得 a2 + b2 = c 。
* 示例 1：

   输入：c = 5
   输出：true
   解释：1 * 1 + 2 * 2 = 5
   示例 2：

   输入：c = 3
   输出：false


   提示：

   0 <= c <= 231 - 1
*/
pub struct Solution {}
impl Solution {
    pub fn find_a_b(c: i32) -> bool {
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i32;
        while left <= right {
            let tmp = left * left + right * right;
            if tmp == c {
                return true;
            } else if tmp > c {
                right -= 1;
            } else {
                left += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_a_b() {
        assert_eq!(Solution::find_a_b(5), true);
        assert_eq!(Solution::find_a_b(3), false);
    }
}
