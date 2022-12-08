struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        // String + &str
        s[n as usize..].to_string() + &s[..n as usize]
    }

    // 300 ms 🥹
    pub fn reverse_left_words_v2(s: String, n: i32) -> String {
        let mut res = String::new();
        for i in n as usize..s.len() {
            res.push(s.chars().nth(i).unwrap());
        }
        for i in 0..n as usize {
            res.push(s.chars().nth(i).unwrap());
        }
        res
    }

    pub fn reverse_left_words_v3(s: String, n: i32) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.rotate_left(n as usize);
        chars.iter().collect()
    }
}
