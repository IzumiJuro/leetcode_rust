use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn invert_tree(root: T) -> T {
        if let Some(node) = root.as_ref() {
            let mut borrow = node.borrow_mut();
            let (left, right) = (
                Self::invert_tree(borrow.left.clone()),
                Self::invert_tree(borrow.right.clone()),
            );
            borrow.left = right;
            borrow.right = left;
        }
        root
    }
}
