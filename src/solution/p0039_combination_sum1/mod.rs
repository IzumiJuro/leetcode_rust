pub struct Solution {}

impl Solution {
    pub fn combination_sum1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = candidates.len();
        let mut result = vec![];
        let mut path = Vec::with_capacity(len);
        Solution::dfs(&candidates, len, 0, target, &mut result, &mut path);
        result
    }

    fn dfs(
        candidates: &[i32],
        len: usize,
        n: usize,
        target: i32,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            result.push(path.to_owned());
            return;
        }

        for i in n..len {
            path.push(candidates[i]);
            Solution::dfs(candidates, len, i, target - candidates[i], result, path);
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test39() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            Solution::combination_sum1(vec![2, 3, 6, 7], 7)
        );
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum1(vec![2, 3, 5], 8)
        );
        let vec: Vec<Vec<i32>> = vec![];
        assert_eq!(vec, Solution::combination_sum1(vec![2], 1));
    }
}
