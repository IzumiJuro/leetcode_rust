use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn preorder_traversal(root: T) -> Vec<i32> {
        fn pre_order(root: T, vec: &mut Vec<i32>) {
            if let Some(node) = root {
                let borrow = node.borrow();
                vec.push(borrow.val);
                pre_order(borrow.left.clone(), vec);
                pre_order(borrow.right.clone(), vec)
            }
        }
        let mut ret = Vec::new();
        pre_order(root, &mut ret);
        ret
    }
}
