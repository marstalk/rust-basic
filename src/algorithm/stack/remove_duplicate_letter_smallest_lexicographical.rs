use std::collections::LinkedList;

pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut res = String::from("");

        // 1. create a counter to find all character's count
        let mut count = vec![0; 256];
        for c in s.chars() {
            count[c as usize] += 1;
        }

        // 2. create a bool array to check if a character is in the stack.
        let mut in_stack = vec![false; 256];

        // 3. iterate through the string
        let mut stack = LinkedList::new();
        for c in s.chars() {
            // 3.1 update counter
            count[c as usize] -= 1;
            // 3.2 if character is already in the stack, skip it.
            if in_stack[c as usize] {
                continue;
            }

            while let Some(top) = stack.back() {
                if *top <= c || count[*top as usize] == 0 {
                    // 3.3 if top is smaller than current OR top there is no more top element, then stop skipping.
                    break;
                }
                // 3.4 pop the top element and update status.
                let top: char = stack.pop_back().unwrap();
                in_stack[top as usize] = false;
            }

            // 3.5 push the current element to stack and update status.
            stack.push_back(c);
            in_stack[c as usize] = true;
        }

        // iterate stack to construct the result.
        while let Some(c) = stack.pop_front() {
            res.push(c);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicate_letters() {
        assert_eq!(
            Solution::remove_duplicate_letters(String::from("bcabc")),
            String::from("abc")
        );
        assert_eq!(
            Solution::remove_duplicate_letters(String::from("cbacdcbc")),
            String::from("acdb")
        );
    }
}
