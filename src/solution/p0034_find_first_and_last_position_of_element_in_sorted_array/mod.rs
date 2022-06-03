pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left_border = Solution::get_left_border(&nums, target);
        let right_border = Solution::get_right_border(&nums, target);
        vec![left_border, right_border]
    }

    fn get_right_border(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == 0 || nums[left - 1] != target {
            -1
        } else {
            (left - 1) as i32
        }
    }

    fn get_left_border(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + ((right - left) >> 1);
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == nums.len() || nums[left] != target {
            -1
        } else {
            left as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
        let vec: Vec<i32> = vec![];
        assert_eq!(vec![-1, -1], Solution::search_range(vec, 0));
    }
}
