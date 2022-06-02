pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let str = String::from("");
        Solution::helper(n, n, str, &mut result);
        result
    }
    pub fn helper(l: i32, r: i32, str: String, result: &mut Vec<String>) {
        if l == 0 && r == 0 {
            result.push(str.to_string());
            return;
        }
        if l > 0 {
            Solution::helper(l - 1, r, str.to_string() + "(", result)
        }
        if l < r {
            Solution::helper(l, r - 1, str.to_string() + ")", result)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
