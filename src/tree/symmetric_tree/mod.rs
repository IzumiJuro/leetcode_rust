use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    // recursion is best!
    pub fn is_symmetric(root: T) -> bool {
        if let Some(node) = root {
            let borrow = node.borrow();
            Self::recur(&borrow.left, &borrow.right)
        } else {
            false
        }
    }
    fn recur(left: &T, right: &T) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(node1), Some(node2)) => {
                node1.borrow().val == node2.borrow().val
                    && Self::recur(&node1.borrow().left, &node2.borrow().right)
                    && Self::recur(&node1.borrow().right, &node2.borrow().left)
            }
            _ => false,
        }
    }

    pub fn test(root: T) -> bool {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            let borrow = node.borrow();
            queue.push_back(borrow.left.clone());
            queue.push_back(borrow.right.clone());
        }
        while !queue.is_empty() {
            let (n1, n2) = (queue.pop_front().unwrap(), queue.pop_front().unwrap());
            match (n1.clone(), n2.clone()) {
                (None, None) => continue,
                (Some(n1), Some(n2)) => {
                    if n1.borrow().val != n2.borrow().val {
                        return false;
                    }
                }
                _ => return false,
            }
            queue.push_back(n1.as_ref().unwrap().borrow().left.clone());
            queue.push_back(n2.as_ref().unwrap().borrow().right.clone());
            queue.push_back(n1.unwrap().borrow().right.clone());
            queue.push_back(n2.unwrap().borrow().left.clone());
        }
        true
    }
}
