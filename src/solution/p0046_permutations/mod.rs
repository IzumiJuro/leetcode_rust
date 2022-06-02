pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut nums = nums;
        Solution::dfs(0, &mut nums, &mut result);
        result
    }

    fn dfs(begin: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if begin == nums.len() {
            result.push(nums.to_owned());
        }
        for i in begin..nums.len() {
            nums.swap(begin, i);
            Self::dfs(begin + 1, nums, result);
            nums.swap(begin, i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2]
            ],
            Solution::permute(vec![1, 2, 3])
        );
    }
}
