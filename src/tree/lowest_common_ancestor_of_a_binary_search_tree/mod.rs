use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: T, p: T, q: T) -> T {
        fn traversal(root: &T, p: i32, q: i32) -> T {
            if let Some(node) = root {
                let val = node.borrow().val;
                if val < p && val < q {
                    traversal(&node.borrow().right, p, q)
                } else if val > p && val > q {
                    traversal(&node.borrow().left, p, q)
                } else {
                    Some(Rc::clone(node))
                }
            } else {
                None
            }
        }
        traversal(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}
