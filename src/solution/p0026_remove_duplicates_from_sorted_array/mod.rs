pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 2 {
            return len as i32;
        }
        let mut index = 0;
        for i in 0..len {
            if nums[i] != nums[index] {
                index += 1;
                nums[index] = nums[i];
            }
        }
        index as i32 + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
        assert_eq!(
            5,
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
        );
    }
}
