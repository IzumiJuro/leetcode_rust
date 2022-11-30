use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type T = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn search_bst(root: Option<T>, val: i32) -> Option<T> {
        if let Some(node) = root {
            let borrow = node.borrow();
            if borrow.val == val {
                return Some(node.clone());
            }
            if borrow.val > val {
                return Self::search_bst(borrow.left.clone(), val);
            }
            if borrow.val < val {
                return Self::search_bst(borrow.right.clone(), val);
            }
        }
        None
    }
}
