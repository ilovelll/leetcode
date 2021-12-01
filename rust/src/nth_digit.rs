// 400. 第N位数字
// 给你一个整数 n ，请你在无限的整数序列 [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...] 中找出并返回第 n 位数字。

//

// 示例 1：

// 输入：n = 3
// 输出：3
// 示例 2：

// 输入：n = 11
// 输出：0
// 解释：第 11 位数字在序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... 里是 0 ，它是 10 的一部分。

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/nth-digit
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        // 位数 --- 范围  --- 个数
        // 1 ---     1~9 ---   9
        // 2 ---   10~99 ---  90
        // 3 --- 100~999 --- 900
        // 先找出区间
        let mut n = n as u64;
        let mut base = 1;
        let mut count = 1;
        while n > 9 * base * count {
            n -= 9 * base * count;
            count += 1;
            base *= 10;
        }
        n -= 1; // 从下标0开始，所以要减1

        // 然后在区间内找出第n个数字, 在第几位
        let (x, y) = (n / count, n % count);
        let mut num = base + x;
        num = num / 10_u64.pow((count - y - 1) as u32);
        (num % 10) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_nth_digit(3), 3);
        assert_eq!(Solution::find_nth_digit(10), 1);
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(12), 1);
        assert_eq!(Solution::find_nth_digit(1000000000), 1);
    }
}
