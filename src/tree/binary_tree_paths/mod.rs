use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::tree::TreeNode;



struct Solution;
type T = Rc<RefCell<TreeNode>>;
impl Solution {
  pub fn binary_tree_paths(root: Option<T>) -> Vec<String> {
    fn dfs(root: Option<T>, cur_path: String, path: &mut Vec<String>) {
      if let Some(node) = root {
        let borrow = node.borrow();
        let value = borrow.val;
        let mut new_path = cur_path;
        new_path.push_str(&value.to_string());
        if borrow.left.is_none() && borrow.right.is_none() {
          path.push(new_path);
          return;
        }
        new_path.push_str("->");
        dfs(borrow.left.clone(), new_path.to_string(), path);
        dfs(borrow.right.clone(), new_path.to_string(), path);
      }
    }
    let mut path = vec![];
    dfs(root, String::from(""), &mut path);
    path
  }
}
