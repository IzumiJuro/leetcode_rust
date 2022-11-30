use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn max_depth(root: T) -> i32 {
        let mut res = 0;
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        while !temp.is_empty() {
            temp = temp
                .iter()
                .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
            res += 1;
        }
        res
    }
}
