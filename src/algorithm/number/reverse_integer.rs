/**
 * 
Given a signed 32-bit integer x, return x with its digits reversed. 
If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned). 

Example 1:
Input: x = 123
Output: 321

Example 2:
Input: x = -123
Output: -321

Example 3:
Input: x = 120
Output: 21
 

Constraints:
-231 <= x <= 231 - 1
 */
pub struct Solution{}
impl Solution{
    pub fn reverse(num: i32) -> i32{
        let mut remainder = num;
        let mut res: i32 = 0;
        while remainder != 0{
            match res.checked_mul(10){
                Some(i) => {
                    res = i;
                    match res.checked_add(remainder % 10){
                        Some(i) => {
                            res = i;
                            remainder /= 10;
                        },
                        None => return 0,
                    }
                },
                None => return 0,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_reverse(){
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}