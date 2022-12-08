use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn insert_into_bst(root: T, val: i32) -> T {
        if let Some(node) = &root {
            {
                let mut node = node.borrow_mut();
                if node.val > val {
                    node.left = Self::insert_into_bst(node.left.take(), val);
                } else {
                    node.right = Self::insert_into_bst(node.right.take(), val);
                }
            }
            root
        } else {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            Some(node)
        }
    }
}
