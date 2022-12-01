use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn path_sum(root: T, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(root: &T, mut target_sum: i32, paths: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            if let Some(node) = root {
                target_sum -= node.borrow().val;
                path.push(node.borrow().val);

                if node.borrow().left.is_none() && node.borrow().right.is_none() && target_sum == 0
                {
                    paths.push(path.to_vec());
                }

                dfs(&node.borrow().left, target_sum, paths, path);
                dfs(&node.borrow().right, target_sum, paths, path);
                path.pop();
            }
        }
        let mut paths = vec![];
        let mut path = vec![];
        dfs(&root, target_sum, &mut paths, &mut path);
        paths
    }
}
