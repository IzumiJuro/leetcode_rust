struct Solution;

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.repeat(2).contains(&b)
    }

    pub fn rotate_string_v2(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        (1..a.len()).any(|i| {
            a[..i] == b[a.len() - i..] && a[i..] == b[..a.len() - i]
        })
    }
} 
