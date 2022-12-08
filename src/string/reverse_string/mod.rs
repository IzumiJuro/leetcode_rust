struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        (0..len / 2).for_each(|i| s.swap(i, len - i - 1))
    }
}
