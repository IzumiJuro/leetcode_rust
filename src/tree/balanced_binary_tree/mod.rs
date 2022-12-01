use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    // 递归
    pub fn is_balanced(root: T) -> bool {
        // 返回当前根节点的最大高度
        fn dfs(root: T) -> i32 {
            if let Some(node) = root {
                let borrow = node.borrow();
                let left = dfs(borrow.left.clone());
                let right = dfs(borrow.right.clone());
                if left == -1 || right == -1 || (left - right).abs() > 1 {
                    return -1;
                }
                return left.max(right) + 1;
            }
            0
        }
        dfs(root) != -1
    }
}
