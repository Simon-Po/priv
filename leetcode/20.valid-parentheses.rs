/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        let lookup = HashMap::from(
            [
                (')','('),
                ('}','{'),
                (']','['),
            ]
        );
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' | '}' | ']' => {
                    if *stack.last().unwrap_or(&' ') == lookup[&c] {
                        stack.pop();
                    } else {
                        return false
                    }
                },
                _ => println!("")

            }
        }
        return stack.len() == 0;
    }
}
// @lc code=end

