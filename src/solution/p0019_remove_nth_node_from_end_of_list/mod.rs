use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        unsafe {
            let mut slow = &mut dummy as *mut Box<ListNode>;
            let mut fast = &mut dummy as *mut Box<ListNode>;

            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod test {
    use crate::util::linked_list::to_list;

    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
    }
}
