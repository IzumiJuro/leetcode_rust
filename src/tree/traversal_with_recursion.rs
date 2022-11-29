use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

pub fn inorder_traversal(root: T) -> Vec<i32> {
    let mut res = vec![];
    in_order(root, &mut res);
    res
}

fn in_order(root: T, vec: &mut Vec<i32>) {
    if let Some(node) = root {
        let borrow = node.borrow();
        in_order(borrow.left.clone(), vec);
        vec.push(borrow.val);
        in_order(borrow.right.clone(), vec);
    }
}

pub fn postorder_traversal(root: T) -> Vec<i32> {
    let mut res = vec![];
    post_order(root, &mut res);
    res
}

fn post_order(root: T, vec: &mut Vec<i32>) {
    if let Some(node) = root {
        let borrow = node.borrow();
        post_order(borrow.left.clone(), vec);
        post_order(borrow.right.clone(), vec);
        vec.push(borrow.val);
    }
}

pub fn preorder_traversal(root: T) -> Vec<i32> {
    let mut res = vec![];
    pre_order(root, &mut res);
    res
}

fn pre_order(root: T, vec: &mut Vec<i32>) {
    if let Some(node) = root {
        let borrow = node.borrow();
        vec.push(borrow.val);
        pre_order(borrow.left.clone(), vec);
        pre_order(borrow.right.clone(), vec);
    }
}



