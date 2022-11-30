use crate::utils::tree::Node;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type T = Option<Rc<RefCell<Node>>>;

impl Solution {
    pub fn level_order(root: T) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut temp = if let Some(node) = root {
            vec![node]
        } else {
            vec![]
        };
        while !temp.is_empty() {
            res.push(temp.iter().map(|n| n.borrow().val).collect());
            temp = temp
                .iter()
                .flat_map(|n| n.borrow().children.clone())
                .flatten()
                .collect()
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let root = Node::from_vec(vec![
            Some(1),
            None,
            Some(3),
            Some(2),
            Some(4),
            None,
            Some(5),
            Some(6),
        ]);
        let res = vec![vec![1], vec![3, 2, 4], vec![5, 6]];
        assert_eq!(Solution::level_order(root), res);

        let root = Node::from_vec(vec![
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            None,
            Some(6),
            Some(7),
            None,
            Some(8),
            None,
            Some(9),
            Some(10),
            None,
            None,
            Some(11),
            None,
            Some(12),
            None,
            Some(13),
            None,
            None,
            Some(14),
        ]);
        let res = vec![
            vec![1],
            vec![2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13],
            vec![14],
        ];
        assert_eq!(Solution::level_order(root), res);
    }
}
