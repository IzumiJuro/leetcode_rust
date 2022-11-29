use std::{cell::RefCell, rc::Rc};
use crate::utils::tree::TreeNode;

struct Solution;
type T = Rc<RefCell<TreeNode>>;

impl Solution {
  pub fn inorder_traversal(root: Option<T>) -> Vec<i32> {
    fn in_order(root: Option<T>, vec: &mut Vec<i32>) {
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
