
pub struct Solution;

impl Solution {
  pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
    let mut set = std::collections::HashSet::with_capacity(nums.len());

    for i in 0..nums.len() {
      if set.get(&nums[i]).is_some() {
        return nums[i];
      }
      set.insert(nums[i]);
    }
    -1
  }
}
