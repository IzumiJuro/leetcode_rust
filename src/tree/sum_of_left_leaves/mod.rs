use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sum_of_left_leaves(root: T) -> i32 {
        let mut res = 0;
        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(node) = stack.remove(0) {
                let borrow = node.borrow();
                if let Some(left) = &node.borrow().left {
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        res += left.borrow().val
                    }
                }
                if borrow.left.is_some() {
                    stack.push(borrow.left.clone())
                }
                if borrow.right.is_some() {
                    stack.push(borrow.right.clone())
                }
            }
        }
        res
    }

    pub fn sum_of_left_leaves_v2(root: T) -> i32 {
        fn sum(root: &T) -> i32 {
            let mut res = 0;
            if let Some(node) = root {
                if let Some(left) = &node.borrow().left {
                    if left.borrow().left.is_none() && left.borrow().right.is_none() {
                        res += left.borrow().val
                    }
                }
                res += sum(&node.borrow().left);
                res += sum(&node.borrow().right);
            }
            res
        }
        sum(&root)
    }
}
