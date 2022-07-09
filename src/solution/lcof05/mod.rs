pub struct Solution;

impl Solution {
  pub fn replace_space(s: String) -> String {
    let mut res = "".to_string();
    for ch in s.chars() {
      if ch == ' ' {
        res += "%20";
      } else {
        res += &ch.to_string();
      }
    }
    res
  }
}
