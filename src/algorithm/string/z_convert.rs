/**
将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。

比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：

P   A   H   N
A P L S I I G
Y   I   R
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。

请你实现这个将字符串进行指定行数变换的函数：

string convert(string s, int numRows);


示例 1：

输入：s = "PAYPALISHIRING", numRows = 3
输出："PAHNAPLSIIGYIR"
示例 2：
输入：s = "PAYPALISHIRING", numRows = 4
输出："PINALSIGYAHRPI"
解释：
P     I    N
A   L S  I G
Y A   H R
P     I
示例 3：

输入：s = "A", numRows = 1
输出："A"


提示：

1 <= s.length <= 1000
s 由英文字母（小写和大写）、',' 和 '.' 组成
1 <= numRows <= 1000
 */

pub struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() == 1 || s.len() == 0 || num_rows == 1 {
            return s;
        }

        let mut res = String::from("");
        let mut used = vec![false; s.len()];

        let j_step = num_rows * 2 - 2;
        for i in 0..num_rows {
            if i >= s.len() as i32 {
                break;
            }

            if !used[i as usize] {
                res.push(s.chars().nth(i as usize).unwrap());
                used[i as usize] = true;
            }

            let mut j = i + j_step;
            loop {
                let idx = j - i * 2;
                if idx >= s.len() as i32 {
                    break;
                }

                if !used[idx as usize] {
                    res.push(s.chars().nth(idx as usize).unwrap());
                    used[idx as usize] = true;
                }

                if j < s.len() as i32 && !used[j as usize] {
                    res.push(s.chars().nth(j as usize).unwrap());
                    used[j as usize] = true;
                }
                j += j_step;
            }
        }
        res
    }

    /**
     * solution from leetcode, custom iterator.
     */
    pub fn convert2(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;

        // 1. iterator A: 0123432101234...
        let iterator_a = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();

        // 2. iterator B: 0123456789...
        let iterator_b = s.chars().into_iter();

        // 3. use zip to iterate A and B
        let iterator_c = iterator_a.zip(iterator_b);

        // 4. iterate and push to corresponding string.
        let mut strings = vec![String::from(""); num_rows];
        iterator_c.into_iter().for_each(|(string_idx, char)| {
            strings[string_idx].push(char);
        });

        // 5. join strings
        strings.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert1() {
        //PAPLSIIGYYIIRR
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(Solution::convert(String::from("A"), 1), "A");
        assert_eq!(Solution::convert(String::from("AB"), 3), "AB");
    }

    #[test]
    fn test_convert2() {
        assert_eq!(
            Solution::convert2("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert2(String::from("PAYPALISHIRING"), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(Solution::convert2(String::from("A"), 1), "A");
        assert_eq!(Solution::convert2(String::from("AB"), 3), "AB");
    }
}
