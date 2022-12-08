struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        s.replace(' ', "%20")
    }

    pub fn replace_space_v2(s: String) -> String {
        let mut res = String::new();
        for char in s.chars() {
            if char == ' ' {
                res.push_str("%20");
            } else {
                res.push(char);
            }
        }
        res
    }


}
