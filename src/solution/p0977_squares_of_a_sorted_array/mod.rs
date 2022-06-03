pub struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut result = vec![0; len];
        let mut slow = 0;
        let mut fast = len;
        let mut index = len;

        while slow < fast {
            index -= 1;
            if nums[slow].abs() > nums[fast - 1].abs() {
                result[index] = nums[slow].pow(2);
                slow += 1;
            } else {
                fast -= 1;
                result[index] = nums[fast].pow(2);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_977() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
    }
}
