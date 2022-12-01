use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn count_nodes(root: T) -> i32 {
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        let mut res = 0;
        while !temp.is_empty() {
            res += temp.len();
            temp = temp
                .iter()
                .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
        }
        res as i32
    }

    pub fn count_nodes_v2(root: T) -> i32 {
        if let Some(node) = root {
            let mut left_depth = 0;
            let mut left = node.borrow().left.clone();
            while let Some(node) = left {
                left_depth += 1;
                left = node.borrow().left.clone();
            }

            let mut right_depth = 0;
            let mut right = node.borrow().right.clone();
            while let Some(node) = right {
                right_depth += 1;
                right = node.borrow().right.clone()
            }

            if left_depth == right_depth {
                return (2 << left_depth) - 1;
            }
            return Self::count_nodes_v2(node.borrow().left.clone())
                + Self::count_nodes_v2(node.borrow().right.clone())
                + 1;
        }
        0
    }
}
