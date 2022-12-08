use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn bst_to_gst(root: T) -> T {
        fn traversal(root: &T, sum: &mut i32) {
            if let Some(node) = root {
                traversal(&node.borrow().right, sum);
                *sum += node.borrow().val;
                node.borrow_mut().val = *sum;
                traversal(&node.borrow().left, sum)
            }
        }
        let mut sum = 0;
        traversal(&root, &mut sum);
        root
    }
}
