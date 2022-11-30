use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

fn preorder_traversal(root: T) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![root];
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            let borrow = node.borrow();
            if borrow.right.is_some() {
                stack.push(borrow.right.clone())
            }
            if borrow.left.is_some() {
                stack.push(borrow.left.clone())
            }
            stack.push(Some(node.clone()));
            stack.push(None);
        } else {
            res.push(stack.pop().unwrap().unwrap().borrow().val);
        }
    }
    res
}

fn inorder_traversal(root: T) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![root];
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            let borrow = node.borrow();
            if borrow.right.is_some() {
                stack.push(borrow.right.clone())
            }
            stack.push(Some(node.clone()));
            stack.push(None);
            if borrow.left.is_some() {
                stack.push(borrow.left.clone())
            }
        } else {
            res.push(stack.pop().unwrap().unwrap().borrow().val);
        }
    }
    res
}

fn postorder_traversal(root: T) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![root];
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            let borrow = node.borrow();
            stack.push(Some(node.clone()));
            stack.push(None);
            if borrow.right.is_some() {
                stack.push(borrow.right.clone())
            }
            if borrow.left.is_some() {
                stack.push(borrow.left.clone())
            }
        } else {
            res.push(stack.pop().unwrap().unwrap().borrow().val);
        }
    }
    res
}

pub fn level_order_traversal_v1(root: T) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let mut res = vec![];
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_front(root);
    }
    while !queue.is_empty() {
        let mut temp = vec![];
        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front().unwrap() {
                let borrow = node.borrow();
                temp.push(borrow.val);
                if borrow.left.clone().is_some() {
                    queue.push_back(borrow.left.clone());
                }
                if borrow.right.clone().is_some() {
                    queue.push_back(borrow.right.clone());
                }
            }
        }
        res.push(temp)
    }
    res
}

pub fn level_order_traversal_v2(root: T) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let mut res = vec![];
    let mut queue = VecDeque::new();
    if let Some(node) = root {
        queue.push_back(node);
    }
    while !queue.is_empty() {
        let mut temp = vec![];
        (0..queue.len()).for_each(|_| {
            if let Some(node) = queue.pop_front() {
                let borrow = node.borrow();
                temp.push(borrow.val);
                if let Some(left) = borrow.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = borrow.right.clone() {
                    queue.push_back(right);
                }
            }
        });
        res.push(temp)
    }
    res
}

pub fn level_order_traversal_v3(root: T) -> Vec<Vec<i32>> {
    let mut temp = if let Some(node) = root {
        vec![node]
    } else {
        vec![]
    };
    let mut res = vec![];
    while !temp.is_empty() {
        res.push(temp.iter().map(|n| n.borrow().val).collect());
        temp = temp
            .iter()
            .flat_map(|n| vec![n.borrow().left.clone(), n.borrow().right.clone()])
            .flatten()
            .collect();
    }
    res
}

#[cfg(test)]
mod test {
    use crate::tree;
    use crate::utils::tree::to_tree;
    #[test]
    fn test() {
        let vec = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(
            super::level_order_traversal_v3(tree![3, 9, 20, null, null, 15, 7]),
            vec
        )
    }
}
