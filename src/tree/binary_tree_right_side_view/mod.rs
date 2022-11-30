use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn right_side_view(root: T) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut res = vec![];
        if let Some(node) = root {
            queue.push_back(node);
        }
        while !queue.is_empty() {
            let val = queue.back().unwrap().borrow().val;
            res.push(val);
            (0..queue.len()).for_each(|_| {
                if let Some(node) = queue.pop_front() {
                    let borrow = node.borrow();
                    if let Some(left) = borrow.left.clone() {
                        queue.push_back(left)
                    }
                    if let Some(right) = borrow.right.clone() {
                        queue.push_back(right)
                    }
                }
            })
        }
        res
    }

    pub fn test(root: T) -> Vec<i32> {
        let mut res = vec![];
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        while !temp.is_empty() {
            res.push(temp.last().unwrap().borrow().val);
            temp = temp
                .iter()
                .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
                .flatten()
                .collect();
        }
        res
    }
}
