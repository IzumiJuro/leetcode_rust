use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn delete_node(root: T, key: i32) -> T {
        if let Some(node) = &root {
            let mut node = node.borrow_mut();
            match node.val.cmp(&key) {
                std::cmp::Ordering::Less => {
                    node.right = Self::delete_node(node.right.take(), key);
                }
                std::cmp::Ordering::Greater => {
                    node.left = Self::delete_node(node.left.take(), key);
                }
                std::cmp::Ordering::Equal => match (node.left.is_none(), node.right.is_none()) {
                    (true, true) => {
                        return None;
                    }
                    (true, false) => {
                        return node.right.take();
                    }
                    (false, true) => {
                        return node.left.take();
                    }
                    (false, false) => {
                        let mut min_node = node.right.clone().unwrap();
                        while min_node.borrow().left.is_some() {
                            let temp = min_node.borrow_mut().left.clone();
                            min_node = temp.unwrap();
                        }
                        min_node.borrow_mut().left = node.left.take();
                        return node.right.take();
                    }
                },
            }
            root.clone()
        } else {
            None
        }
    }
}
