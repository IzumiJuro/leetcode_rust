use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
type T = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn preorder_traversal(root: Option<T>) -> Vec<i32> {
        fn pre_order(root: Option<T>, vec: &mut Vec<i32>) {
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
