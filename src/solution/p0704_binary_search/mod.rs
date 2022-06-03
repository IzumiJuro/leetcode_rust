pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        if nums.len() == 0 {
            return -1;
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32;
            }
        }
        if nums[left] == target {
            return left as i32;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_704() {
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
    }
}
