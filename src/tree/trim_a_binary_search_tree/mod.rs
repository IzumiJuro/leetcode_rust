use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn trim_bst(root: T, low: i32, high: i32) -> T {
        if let Some(node) = &root {
            {
                let mut node = node.borrow_mut();
                if node.val < low {
                    return Self::trim_bst(node.right.take(), low, high);
                } else if node.val > high {
                    return Self::trim_bst(node.left.take(), low, high);
                } else {
                    node.left = Self::trim_bst(node.left.take(), low, high);
                    node.right = Self::trim_bst(node.right.take(), low, high);
                }
            }
            root
        } else {
            None
        }
    }
}
