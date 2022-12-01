use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn check_tree(root: T) -> bool {
        if let Some(node) = root {
            let borrow = node.borrow();
            let left_child = borrow.left.as_ref().unwrap().borrow().val;
            let right_child = borrow.right.as_ref().unwrap().borrow().val;
            borrow.val == left_child + right_child
        } else {
            false
        }
    }
}
