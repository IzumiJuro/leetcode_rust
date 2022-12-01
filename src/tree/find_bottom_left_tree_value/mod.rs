use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_bottom_left_value(root: T) -> i32 {
        let mut res = vec![];
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        while !temp.is_empty() {
            res = temp.iter().map(|n| n.borrow().val).collect();
            temp = temp
                .iter()
                .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
        }
        return *res.first().unwrap();
    }
}
