use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn get_minimum_difference(root: T) -> i32 {
        fn in_order(root: &T, nums: &mut Vec<i32>) {
            if let Some(node) = root {
                in_order(&node.borrow().left, nums);
                nums.push(node.borrow().val);
                in_order(&node.borrow().right, nums);
            }
        }
        let mut res = vec![];
        in_order(&root, &mut res);
        (1..res.len()).map(|x| res[x] - res[x-1]).min().unwrap()
    }

    pub fn get_minimum_difference_v2(root: T) -> i32 {
        fn traversal(root: &T, min: &mut i32, pre: &mut Option<i32>) {
            if let Some(node) = root {
                traversal(&node.borrow().left, min, pre);
                if let Some(pre_val) = pre {
                    *min = (*min).min((node.borrow().val - *pre_val).abs());
                }
                *pre = Some(node.borrow().val);
                traversal(&node.borrow().right, min, pre);
            }
        }
        let mut min = i32::MAX;
        let mut pre = None;
        traversal(&root, &mut min, &mut pre);
        min
    }
}
