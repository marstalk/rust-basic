/**
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".
 */

pub struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strings: Vec<String>) -> String {
        let mut res = String::from("");
        let mut idx = 0 as usize;
        'a: for char in strings[0].chars() {
            for str in strings[1..strings.len()].iter() {
                if idx >= str.len() || str.chars().nth(idx).unwrap() != char {
                    break 'a;
                }
            }
            res.push(char);

            idx += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strings = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let res = Solution::longest_common_prefix(strings);
        assert_eq!(res, "fl");
    }
}
