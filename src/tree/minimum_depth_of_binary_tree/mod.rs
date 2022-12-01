use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn min_depth_v1(root: T) -> i32 {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back((node, 1))
        }
        while !queue.is_empty() {
            let (node, cur_depth) = queue.pop_front().unwrap();
            let borrow = node.borrow();
            let left = borrow.left.clone();
            let right = borrow.right.clone();
            if left.is_none() && right.is_none() {
                return cur_depth;
            }
            if let Some(left) = left {
                queue.push_back((left, cur_depth + 1))
            }
            if let Some(right) = right {
                queue.push_back((right, cur_depth + 1))
            }
        }
        0
    }

    pub fn min_depth_v2(root: T) -> i32 {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        let mut res = 0;
        while !queue.is_empty() {
            res += 1;
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    let borrow = node.borrow();
                    if borrow.left.is_none() && borrow.right.is_none() {
                        return res;
                    }
                    if let Some(left) = borrow.left.clone() {
                        queue.push_back(left)
                    }
                    if let Some(right) = borrow.right.clone() {
                        queue.push_back(right)
                    }
                }
            }
        }
        res
    }
}
