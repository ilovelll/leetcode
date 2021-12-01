// 1. 两数之和
// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。

// 你可以按任意顺序返回答案。

//

// 示例 1：

// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
// 示例 2：

// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]
// 示例 3：

// 输入：nums = [3,3], target = 6
// 输出：[0,1]
//

// 提示：

// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// 只会存在一个有效答案
// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/two-sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash = HashMap::with_capacity(nums.len());
        for (idx, value) in nums.iter().enumerate() {
            match hash.get(&(target - value)) {
                None => {
                    hash.insert(value, idx);
                }
                Some(idx2) => {
                    return vec![*idx2 as i32, idx as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![] as Vec<i32>, Solution::two_sum(vec![3, 7, 11, 15], 9));
        assert_eq!(vec![0, 2], Solution::two_sum(vec![2, 7, 11, 15], 13));
    }
}
