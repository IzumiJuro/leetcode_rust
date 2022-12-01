use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: T) -> Vec<i32> {
        fn in_order(root: T, vec: &mut Vec<i32>) {
            if let Some(node) = root {
                let borrow = node.borrow();
                in_order(borrow.left.clone(), vec);
                vec.push(borrow.val);
                in_order(borrow.right.clone(), vec);
            }
        }
        let mut res = vec![];
        in_order(root, &mut res);
        res
    }
}
