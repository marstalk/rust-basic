/***
 * input str: String, pattern: String -> bool,
 * pattern could be :
 * 1. '.' any character.
 * 2. '*' zero or more of the preceding element.
 *
 * return true if str match the pattern.
 */
pub struct Solution {
    // 0: not calculated
    // -1: false
    // 1: true
}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut mem: Vec<Vec<i8>> = vec![vec![0; p.len()]; s.len()];
        for i in 0..s.len() {
            for j in 0..p.len() {
                mem[i][j] = 0;
            }
        }
        Solution::dp(&s, 0, &p, 0, &mut mem) == 1
    }

    fn dp(s: &str, i: usize, p: &str, j: usize, mem: &mut Vec<Vec<i8>>) -> i8 {
        // base 1: p is empty
        if j == p.len() {
            return match i == s.len() {
                true => 1,
                false => -1,
            };
        }
        // base 2: s is empty, p is not empty: * can match zero.
        if i == s.len() {
            if (p.len() - j) % 2 == 1 {
                return -1;
            }

            let mut k = j;
            while k + 1 < p.len() {
                if p.chars().nth(k + 1).unwrap() != '*' {
                    return -1;
                }
                k += 2;
            }
            return 1;
        }
        // mem
        if mem[i][j] != 0 {
            return mem[i][j];
        }
        // current
        let res;
        if s.chars().nth(i).unwrap() == p.chars().nth(j).unwrap()
            || p.chars().nth(j).unwrap() == '.'
        {
            // if p[j+1] is *
            if j < p.len() - 1 && p.chars().nth(j + 1).unwrap() == '*' {
                // * match zero
                let a = Solution::dp(s, i, p, j + 2, mem);
                // * match one
                let b = Solution::dp(s, i + 1, p, j, mem);
                res = match (a, b) {
                    (1, _) => 1,
                    (_, 1) => 1,
                    _ => -1,
                };
            } else {
                res = Solution::dp(s, i + 1, p, j + 1, mem);
            }
        } else {
            // if p[j+1] is *
            if j < p.len() - 1 && p.chars().nth(j + 1).unwrap() == '*' {
                // * match zero
                res = Solution::dp(s, i, p, j + 2, mem);
            } else {
                res = -1;
            }
        }
        // update mem
        mem[i][j] = res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }
}
