use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut root = ListNode { val: 0, next: head };
        let mut head = &mut root;
        while let Some(node) = head.next.take() {
            if node.val == val {
                head.next = node.next;
            } else {
                head.next = Some(node);
                head = head.next.as_mut().unwrap();
            }
        }
        root.next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::linked_list::to_list;
    #[test]
    fn test_203() {
        let vec = vec![1, 2, 6, 3, 4, 5, 6];
        assert_eq!(
            Solution::remove_elements(to_list(vec), 6),
            to_list(vec![1, 2, 3, 4, 5])
        );
    }
}
