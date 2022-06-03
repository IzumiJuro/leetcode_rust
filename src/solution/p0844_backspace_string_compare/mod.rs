pub struct Solution {}

impl Solution {
    pub fn backspace_string(s: String, t: String) -> bool {
        let helper = |s: String| -> String {
            let mut result = String::new();
            for c in s.chars() {
                if c == '#' {
                    result.pop();
                } else {
                    result.push(c);
                }
            }
            result
        };
        helper(s) == helper(t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_844() {
        assert_eq!(
            true,
            Solution::backspace_string("ab#c".to_string(), "ad#c".to_string())
        );
        assert_eq!(
            true,
            Solution::backspace_string("ab##".to_string(), "c#d#".to_string())
        );
        assert_eq!(
            false,
            Solution::backspace_string("a#c".to_string(), "b".to_string())
        );
    }
}
