/*
 * @lc app=leetcode id=20 lang=typescript
 *
 * [20] Valid Parentheses
 */

// @lc code=start
function isValid(s: string): boolean {
  const openingBrackets = new Set(["(", "{", "["]);
  const closingMap = new Map([
    [")", "("],
    ["}", "{"],
    ["]", "["],
  ]);
  
  const stack: string[] = [];

  for (const c of s) {
    if (openingBrackets.has(c)) {
      stack.push(c);
    } else {
      if (stack.length === 0 || stack[stack.length - 1] !== closingMap.get(c)) {
        return false;
      }
      stack.pop();
    }
  }

  return stack.length === 0;
}
// @lc code=end
