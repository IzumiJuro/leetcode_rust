pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(&nums[i]);
            if set.len() > k as usize{
                set.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,2,3,1], 3))
    }
}