// 20. 有效的括号
// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

// 有效字符串需满足：

// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
//

// 示例 1：

// 输入：s = "()"
// 输出：true
// 示例 2：

// 输入：s = "()[]{}"
// 输出：true
// 示例 3：

// 输入：s = "(]"
// 输出：false
// 示例 4：

// 输入：s = "([)]"
// 输出：false
// 示例 5：

// 输入：s = "{[]}"
// 输出：true

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/valid-parentheses
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn pair(c: u8) -> u8 {
            match c {
                b'{' => b'}',
                b'[' => b']',
                b'(' => b')',
                _ => c,
            }
        }
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        for i in s.bytes() {
            match i {
                b'{' | b'[' | b'(' => {
                    stack.push(pair(i));
                }
                b'}' | b']' | b')' => {
                    let pop = stack.pop().unwrap_or(0);
                    if pop != i {
                        return false;
                    }
                }
                _ => {}
            }
        }
        return stack.is_empty();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
        assert_eq!(false, Solution::is_valid("(]".to_string()));
        assert_eq!(false, Solution::is_valid("([)]".to_string()));
        assert_eq!(true, Solution::is_valid("{[]}".to_string()));
    }
}
