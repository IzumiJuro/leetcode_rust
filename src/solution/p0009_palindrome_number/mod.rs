pub struct Solution {}

impl Solution {
    pub fn is_palindrome1(x: i32) -> bool {
        if x < 0 || (x % 10 < 0 && x != 0) {
            return false;
        }
        let mut n = 0;
        let mut num = x;

        while num > 0 {
            n = n * 10 + num % 10;
            num /= 10;
        }

        match n == x {
            true => true,
            false => false,
        }
    }
    pub fn is_palindrome2(x: i32) -> bool {
        if x < 0 || (x % 10 < 0 && x != 0) {
            return false;
        }
        match x.to_string().chars().rev().collect::<String>() == x.to_string() {
            true => true,
            false => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(true, Solution::is_palindrome1(121));
        assert_eq!(false, Solution::is_palindrome1(-121));
        assert_eq!(false, Solution::is_palindrome1(10));
        assert_eq!(true, Solution::is_palindrome2(121));
        assert_eq!(false, Solution::is_palindrome2(-121));
        assert_eq!(false, Solution::is_palindrome2(10));
    }
}
