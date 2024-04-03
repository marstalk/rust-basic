use std::collections::LinkedList;

/**
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

示例 1：
输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
示例 2：
输入：n = 1
输出：["()"]
 */

pub fn find_all_valid_bracket(n: i32) -> Vec<String> {
    let mut result = vec![];
    let mut path = LinkedList::new();
    backtracing(&mut path, &mut result, n);
    result
}

fn backtracing(path: &mut LinkedList<char>, result: &mut Vec<String>, n: i32) {
    // base
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for c in path.iter() {
        match c {
            '(' => cnt1 += 1,
            ')' => cnt2 += 1,
            _ => {}
        }
    }

    // found result, but can continue. so there is no return.
    if cnt1 == n && cnt2 == n {
        let path_str: String = path.iter().collect();
        if valid_bracket(&path_str) {
            result.push(path_str);
        }
    }

    if cnt1 < n {
        path.push_back('(');
        backtracing(path, result, n);
        path.pop_back();
    }
    if cnt2 < n {
        path.push_back(')');
        backtracing(path, result, n);
        path.pop_back();
    }
}

pub fn valid_bracket(bracket: &str) -> bool {
    let mut stack = LinkedList::new();
    for c in bracket.chars() {
        match c {
            '(' => stack.push_front(c),
            ')' => match stack.pop_front() {
                Some(_) => {}
                None => return false,
            },
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bracket() {
        assert!(valid_bracket("((()))"));
        assert!(valid_bracket("()()()"));
        assert!(!valid_bracket("(()()))"));
        assert!(!valid_bracket("()()()("));
        assert!(!valid_bracket(")()()("));
        assert!(!valid_bracket("((("));
    }

    #[test]
    fn test_find_all_valid_bracket() {
        assert_eq!(
            find_all_valid_bracket(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(find_all_valid_bracket(1), vec!["()"]);
    }
}
