use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn average_of_levels(root: T) -> Vec<f64> {
        let mut res = vec![];
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        while !temp.is_empty() {
            let len = temp.len();
            let sum = temp.iter().map(|n| n.borrow().val as f64).sum::<f64>();
            res.push(sum / len as f64);
            temp = temp
                .iter()
                .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
        }
        res
    }
}
