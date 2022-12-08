use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: T, p: T, q: T) -> T {
        fn traversal(root: &T, p: i32, q: i32) -> T {
            if let Some(node) = root {
                if node.borrow().val == p || node.borrow().val == q {
                    return Some(Rc::clone(node));
                }
                let left = traversal(&node.borrow().left, p, q);
                let right = traversal(&node.borrow().right, p, q);
                if left.is_none() {
                    right
                } else if right.is_none() {
                    left
                } else {
                    root.clone()
                }
            } else {
                None
            }
        }
        traversal(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}
