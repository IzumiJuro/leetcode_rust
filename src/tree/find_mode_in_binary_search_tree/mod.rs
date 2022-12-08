use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_mode(root: T) -> Vec<i32> {
        fn traversal(root: &T, nums: &mut Vec<i32>) {
            if let Some(node) = root {
                traversal(&node.borrow().left, nums);
                nums.push(node.borrow().val);
                traversal(&node.borrow().right, nums);
            }
        }
        let mut nums = vec![];
        traversal(&root, &mut nums);
        let mut res = vec![];
        let mut max_count = 0;
        let mut count = 0;
        let mut pre = None;
        for num in nums {
            if let Some(pre_val) = pre {
                if num == pre_val {
                    count += 1;
                } else {
                    count = 1;
                }
            } else {
                count = 1;
            }
            match count.cmp(&max_count) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    res.push(num);
                }
                std::cmp::Ordering::Greater => {
                    res = vec![num];
                    max_count = count;
                }
            }
            pre = Some(num);
        }
        res
    }
}
