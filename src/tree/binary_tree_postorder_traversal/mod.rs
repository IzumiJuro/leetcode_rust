use std::{cell::RefCell, rc::Rc};
use crate::utils::tree::TreeNode;

struct Solution;
type T = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn postorder_traversal(root: Option<T>) -> Vec<i32> {
        fn post_order(root: Option<T>, vec: &mut Vec<i32>) {
            if let Some(node) = root {
                let borrow = node.borrow();
                post_order(borrow.left.clone(), vec);
                post_order(borrow.right.clone(), vec);
                vec.push(borrow.val);
            }
        }
        let mut res = vec![];
        post_order(root, &mut res);
        res
    }
}

#[cfg(test)]
mod test {
    use crate::utils::tree::to_tree;
    use crate::tree;

    use super::*;

    #[test]
    fn postorder_test() {
        assert_eq!(
            Solution::postorder_traversal(tree!([1,null,2,3])),
            vec![3,2,1]
        )
    }
}
