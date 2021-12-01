// 7. 整数反转
// 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。

// 如果反转后整数超过 32 位的有符号整数的范围 [−231,  231 − 1] ，就返回 0。

// 假设环境不允许存储 64 位整数（有符号或无符号）。
//

// 示例 1：

// 输入：x = 123
// 输出：321
// 示例 2：

// 输入：x = -123
// 输出：-321
// 示例 3：

// 输入：x = 120
// 输出：21
// 示例 4：

// 输入：x = 0
// 输出：0
//

// 提示：

// -231 <= x <= 231 - 1

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/reverse-integer
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        use std::i32;
        let mut curr: i32 = 0;
        let mut prev: i32 = x;
        while prev != 0 {
            let (tmp, flag) = curr.overflowing_mul(10);
            if flag {
                return 0;
            }
            let (tmp, flag) = tmp.overflowing_add(prev % 10);
            if flag {
                return 0;
            }
            curr = tmp;
            prev = prev / 10;
        }
        curr
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(123, Solution::reverse(321));
        assert_eq!(0, Solution::reverse(0));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(0, Solution::reverse(i32::MAX));
    }
}
