use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution{
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> T {
        let max_index = (0..nums.len()).max_by_key(|&i| nums[i]).unwrap();
        let mut root = TreeNode::new(nums[max_index]);
        if max_index > 0 {
            root.left = Self::construct_maximum_binary_tree(nums[0..max_index].to_vec());
        }
        if max_index < nums.len() - 1 {
            root.right = Self::construct_maximum_binary_tree(nums[max_index + 1..].to_vec());
        }
        Some(Rc::new(RefCell::new(root)))
    }
}
