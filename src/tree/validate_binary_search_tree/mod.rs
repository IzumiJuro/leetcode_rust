use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_valid_bst(root: T) -> bool {
        fn validate(root: &T, min: i64, max: i64) -> bool {
            if let Some(node) = root {
                let borrow = node.borrow();
                let val = borrow.val as i64;
                if val <= min || val >= max {
                    return false;
                }
                validate(&borrow.left, min, val) && validate(&borrow.right, val, max)
            } else {
                true
            }
        }
        validate(&root, i64::MIN, i64::MAX)
    }
}
