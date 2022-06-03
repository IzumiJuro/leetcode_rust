pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let (mut left, mut right) = (1, num);
        while left <= right {
            let mid = left + (right - left) / 2;
            let target = mid * mid;
            if target == num {
                return true;
            } else if target < num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_367() {
        assert_eq!(true, Solution::is_perfect_square(16));
        assert_eq!(false, Solution::is_perfect_square(14));
    }
}
