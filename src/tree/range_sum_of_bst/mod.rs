use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn range_sum_bst(root: T, low: i32, high: i32) -> i32 {
        let mut stack = vec![root];
        let mut res = 0;

        while let Some(node) = stack.pop() {
            if let Some(n) = node {
                let borrow = n.borrow();
                if borrow.val <= high && borrow.val >= low {
                    res += borrow.val;
                }
                stack.push(borrow.left.clone());
                stack.push(borrow.right.clone());
            }
        }
        res
    }
}
