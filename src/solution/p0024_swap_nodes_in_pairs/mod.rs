use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut tail = &mut res;
        while let Some(mut first) = head.take() {
            if let Some(mut second) = first.next.take() {
                head = second.next.take();
                tail = &mut tail.insert(second).next;
                tail = &mut tail.insert(first).next;
            } else {
                let _ = tail.insert(first);
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::util::linked_list::to_list;

    use super::*;

    #[test]
    fn test_24() {
        assert_eq!(
            to_list(vec![2, 1, 4, 3]),
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4]))
        );
    }
}
