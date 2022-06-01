pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return mid as i32;
                }
                right = mid - 1;
            }
        }
        return left as i32;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_35() {
        let vector = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vector, 5), 2);
        let vector = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vector, 2), 1);
        let vector = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(vector, 7), 4);
    }
}
