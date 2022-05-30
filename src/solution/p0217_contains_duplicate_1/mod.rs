pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true
            }
            set.insert(num);
        }
        false
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert_eq!(true, Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
}
