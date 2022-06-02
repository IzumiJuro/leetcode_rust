pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = candidates.len();
        let mut path = vec![];
        let mut result = vec![];
        candidates.sort();
        Self::dfs(&candidates, target, 0, len, &mut path, &mut result);
        result
    }

    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        n: usize,
        len: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 && path.len() > 0 {
            result.push(path.to_owned());
            return;
        }
        for i in n..len {
            if target < candidates[i] {
                break;
            }
            if i > n && candidates[i] == candidates[i - 1] {
                continue;
            }
            path.push(candidates[i]);
            Self::dfs(candidates, target - candidates[i], i + 1, len, path, result);
            path.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test40() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );
        assert_eq!(
            vec![vec![1, 2, 2], vec![5]],
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)
        );
    }
}
