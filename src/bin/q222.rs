/*
 * @lc app=leetcode.cn id=222 lang=rust
 *
 * [222] 完全二叉树的节点个数
 *
 * https://leetcode-cn.com/problems/count-complete-tree-nodes/description/
 *
 * algorithms
 * Medium (40.89%)
 * Total Accepted:    2.8K
 * Total Submissions: 6.8K
 * Testcase Example:  '[1,2,3,4,5,6]'
 *
 * 给出一个完全二叉树，求出该树的节点个数。
 *
 * 说明：
 *
 *
 * 完全二叉树的定义如下：在完全二叉树中，除了最底层节点可能没填满外，其余每层节点数都达到最大值，并且最下面一层的节点都集中在该层最左边的若干位置。若最底层为第
 * h 层，则该层包含 1~ 2^h 个节点。
 *
 * 示例:
 *
 * 输入:
 *    1
 *   / \
 *  2   3
 * / \  /
 * 4  5 6
 *
 * 输出: 6
 *
 */
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn count_nodes(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    count_nodes(&node.borrow().left) + count_nodes(&node.borrow().right) + 1
                }
                None => 0,
            }
        }
        count_nodes(&root)
    }
}

fn main() {}

struct Solution {}
