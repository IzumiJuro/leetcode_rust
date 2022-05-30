pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            if let Some(k) = map.get(&(target - nums[i])) {
                if *k != i {
                    return vec![*k as i32, i as i32]
                }
            }
            map.insert(nums[i], i);

        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(vec![0,1], Solution::two_sum(vec![2,7,11,15], 9))
    }
}