use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_tilt(root: T) -> i32 {
        fn dfs(root: T) -> (i32, i32) {
            if let Some(node) = root {
                let borrow = node.borrow();
                let x = dfs(borrow.left.clone());
                let y = dfs(borrow.right.clone());
                (x.0 + y.0 + (x.1 - y.1).abs(), x.1 + y.1 + borrow.val)
            } else {
                (0, 0)
            }
        }
        dfs(root).0
    }
}
