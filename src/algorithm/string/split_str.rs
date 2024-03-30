/**
 *
给定一个非空字符串S，其被N个‘-’分隔成N+1的子串，给定正整数K，要求除第一个子串外，其余的子串每K个字符组成新的子串，并用‘-’分隔。
对于新组成的每一个子串，如果它含有的小写字母比大写字母多，则将这个子串的所有大写字母转换为小写字母；
反之，如果它含有的大写字母比小写字母多，则将这个子串的所有小写字母转换为大写字母；
大小写字母的数量相等时，不做转换。

输入描述:
输入为两行，第一行为参数K，第二行为字符串S。
输出描述:
输出转换后的字符串。
示例1
输入
3
12abc-abCABc-4aB@
输出
12abc-abc-ABC-4aB-@
说明
子串为12abc、abCABc、4aB@，第一个子串保留，后面的子串每3个字符一组为abC、ABc、4aB、@，abC中小写字母较多，转换为abc，ABc中大写字母较多，转换为ABC，4aB中大小写字母都为1个，不做转换，@中没有字母，连起来即12abc-abc-ABC-4aB-@
示例2
输入
12
12abc-abCABc-4aB@
输出
12abc-abCABc4aB@
说明
子串为12abc、abCABc、4aB@，第一个子串保留，后面的子串每12个字符一组为abCABc4aB@，这个子串中大小写字母都为4个，不做转换，连起来即12abc-abCABc4aB@
 */
pub struct Solution {}
impl Solution {
    pub fn split(i: i32, s: String) -> String {
        let mut res = String::from("");

        let mut sub_str = String::from("");
        let mut small_cnt = 0;
        let mut cap_cnt = 0;
        let mut first_sub = true;
        for c in s.chars() {
            if c == '-' {
                if first_sub {
                    res.push_str(&sub_str);
                    sub_str = String::from("");
                }
                first_sub = false;
            } else {
                if first_sub {
                    sub_str.push(c);
                } else {
                    if sub_str.len() as i32 == i {
                        // new substring finished.
                        if small_cnt > cap_cnt {
                            sub_str = sub_str.to_lowercase();
                        } else if small_cnt < cap_cnt {
                            sub_str = sub_str.to_uppercase();
                        }
                        res.push('-');
                        res.push_str(&sub_str);

                        // reinitialize substring
                        sub_str = String::from("");
                        small_cnt = 0;
                        cap_cnt = 0;
                    }

                    if c.is_lowercase() {
                        small_cnt += 1;
                    } else if c.is_uppercase() {
                        cap_cnt += 1;
                    } // for special charactor like @#$, doesn't increase small_cnt or cap_cnt.
                    sub_str.push(c);
                }
            }
        }

        // ending
        res.push('-');
        res.push_str(&sub_str);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // 12abc-abc-bc4a
        // 12abc-abc-bc4a-@
        // 12abc-abc-AB-c4a-B@
        // 12abc-abc-ABC-4aB@
        assert_eq!(
            Solution::split(3, "12abc-abCABc-4aB@".to_string()),
            "12abc-abc-ABC-4aB-@"
        );
    }

    #[test]
    fn test_solution2() {
        assert_eq!(
            Solution::split(12, "12abc-abCABc-4aB@".to_string()),
            "12abc-abCABc4aB@"
        );
    }
}
