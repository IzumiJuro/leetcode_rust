struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        (1..s.len()).filter(|&i| s.len() % i == 0).any(|i| s[..i].repeat(s.len() / i) == s)
    }
}
