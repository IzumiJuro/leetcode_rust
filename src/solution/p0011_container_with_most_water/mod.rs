
pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut ans) = (0 as usize, height.len() - 1 as usize, -1);
        while left < right {
            ans = ans.max(height[left].min(height[right]) * (right as i32 - left as i32));
            if height[left] >= height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        ans
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}