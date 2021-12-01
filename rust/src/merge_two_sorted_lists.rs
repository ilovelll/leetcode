// 21. 合并两个有序链表
// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。

// 示例 1：

// 输入：l1 = [1,2,4], l2 = [1,3,4]
// 输出：[1,1,2,3,4,4]
// 示例 2：

// 输入：l1 = [], l2 = []
// 输出：[]
// 示例 3：

// 输入：l1 = [], l2 = [0]
// 输出：[0]

// 提示：

// 两个链表的节点数目范围是 [0, 50]
// -100 <= Node.val <= 100
// l1 和 l2 均按 非递减顺序 排列

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut t1 = list1;
        let mut t2 = list2;
        let mut new_list = Box::new(ListNode::new(0));
        let mut curr_list = &mut new_list;
        while let (Some(v1), Some(v2)) = (t1.as_mut(), t2.as_mut()) {
            if v1.val < v2.val {
                curr_list.as_mut().next = Some(Box::new(ListNode::new(v1.val)));
                t1 = v1.next.take();
            } else {
                curr_list.as_mut().next = Some(Box::new(ListNode::new(v2.val)));
                t2 = v2.next.take();
            }
            curr_list = curr_list.next.as_mut().unwrap();
        }
        if t1.is_some() {
            curr_list.as_mut().next = t1;
        } else {
            curr_list.as_mut().next = t2;
        }
        new_list.next.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
