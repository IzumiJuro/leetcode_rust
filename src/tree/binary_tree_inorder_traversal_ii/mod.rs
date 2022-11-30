use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn level_order_bottom(root: T) -> Vec<Vec<i32>> {
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        let mut res = vec![];
        while !temp.is_empty() {
            res.push(temp.iter().map(|n| n.borrow().val).collect());
            temp = temp
                .iter()
                .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
        }
        res.into_iter().rev().collect()
    }

    pub fn test(root: T) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut res = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        while !queue.is_empty() {
            let mut temp = vec![];
            (0..queue.len()).for_each(|_| {
                if let Some(node) = queue.pop_front() {
                    let borrow = node.borrow();
                    if let Some(left) = borrow.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = borrow.right.clone() {
                        queue.push_back(right);
                    }
                    temp.push(borrow.val);
                }
            });
            res.push_front(temp)
        }
        res.into_iter().collect()
    }
}
