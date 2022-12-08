struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        (0..s.len()).step_by(2 * k as usize).for_each(|i| {
            let len = if s.len() - i > k as usize {
                k
            } else {
                (s.len() - i) as i32
            };
            s[i..(i + len as usize)].reverse();
        });
        s.into_iter().collect()
    }
}
