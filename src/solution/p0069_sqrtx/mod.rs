pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let (mut left, mut right): (i64, i64) = (0, x + 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let target = mid * mid;
            if target == x {
                return mid as i32;
            } else if target > x {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        right as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}
