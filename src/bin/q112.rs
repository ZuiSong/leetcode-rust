//给定一个二叉树和一个目标和，判断该树中是否存在根节点到叶子节点的路径，这条路径上所有节点值相加等于目标和。
//
// 说明: 叶子节点是指没有子节点的节点。
//
// 示例:
//给定如下二叉树，以及目标和 sum = 22，
//
//               5
//             / \
//            4   8
//           /   / \
//          11  13  4
//         /  \      \
//        7    2      1
//
//
// 返回 true, 因为存在目标和为 22 的根节点到叶子节点的路径 5->4->11->2。
// Related Topics 树 深度优先搜索

use std::cell::RefCell;
use std::rc::Rc;

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match node {
            None => false,
            Some(n) => {
                let n = n.borrow();
                if n.right.is_none() && n.left.is_none() && n.val == sum {
                    return true;
                }
                let new_target = sum - n.val;
                Self::helper(&n.left, new_target) || Self::helper(&n.right, new_target)
            }
        }
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::helper(&root, sum)
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {}

struct Solution {}
