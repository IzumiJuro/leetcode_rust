use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution{
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> T {
        if inorder.is_empty() {
            return None;
        }
        let len = postorder.len();
        let val = postorder.last().unwrap();
        let index = inorder.iter().position(|x| x == val).unwrap();

        let mut root = TreeNode::new(*val);

        if index > 0 {
            root.left = Self::build_tree(inorder[0..index].to_vec(), postorder[0..index].to_vec());
        }
        if index < len - 1 {
            root.right = Self::build_tree(inorder[index + 1..].to_vec(), postorder[index..len - 1].to_vec());
        }

        Some(Rc::new(RefCell::new(root)))
    }
}
