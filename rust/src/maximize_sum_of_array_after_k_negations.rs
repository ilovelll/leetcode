// 1005. K 次取反后最大化的数组和
// 给你一个整数数组 nums 和一个整数 k ，按以下方法修改该数组：

// 选择某个下标 i 并将 nums[i] 替换为 -nums[i] 。
// 重复这个过程恰好 k 次。可以多次选择同一个下标 i 。

// 以这种方式修改数组后，返回数组 可能的最大和 。

 

// 示例 1：

// 输入：nums = [4,2,3], k = 1
// 输出：5
// 解释：选择下标 1 ，nums 变为 [4,-2,3] 。
// 示例 2：

// 输入：nums = [3,-1,0,2], k = 3
// 输出：6
// 解释：选择下标 (1, 2, 2) ，nums 变为 [3,1,0,2] 。
// 示例 3：

// 输入：nums = [2,-3,-1,5,-4], k = 2
// 输出：13
// 解释：选择下标 (1, 4) ，nums 变为 [2,3,-1,5,4] 。
 

// 提示：

// 1 <= nums.length <= 104
// -100 <= nums[i] <= 100
// 1 <= k <= 104
pub struct Solution {}
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by_key(|x| x.abs());
        nums.reverse();
        let mut k = k;
        let mut idx = 0;
        while k > 0 {
            // 这里还可以利用k和nums.len()的关系和数值的关系进行优化，提前结束循环
            while idx < nums.len() && nums[idx] >= 0 {
                idx += 1;
            }
            if idx == nums.len() {
                idx -= 1;
            }
            nums[idx] = -nums[idx];
            k -= 1;
        }
        nums.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4,2,3], 1), 5);
        assert_eq!(Solution::largest_sum_after_k_negations(vec![3,-1,0,2], 3), 6);
        assert_eq!(Solution::largest_sum_after_k_negations(vec![2,-3,-1,5,-4], 2), 13);
    }
}
