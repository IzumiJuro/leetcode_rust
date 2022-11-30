use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type T = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<T> {
        Self::sorted_slice_to_bst(&nums)
    }
    pub fn sorted_slice_to_bst(nums: &[i32]) -> Option<T> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let mut root = TreeNode::new(nums[mid]);
        root.left = Self::sorted_slice_to_bst(&nums[..mid]);
        root.right = Self::sorted_slice_to_bst(&nums[mid + 1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}
