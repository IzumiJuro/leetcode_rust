use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn has_path_sum(root: T, target_sum: i32) -> bool {
        if let Some(node) = root {
            let mut p = node.borrow_mut();
            return (p.left.is_none() && p.right.is_none() && p.val == target_sum)
                || Self::has_path_sum(p.left.take(), target_sum - p.val)
                || Self::has_path_sum(p.right.take(), target_sum - p.val);
        }
        false
    }
}
