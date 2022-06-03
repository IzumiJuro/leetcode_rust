pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[index] = nums[i];
                index += 1;
            }
        }

        for i in index..nums.len() {
            nums[i] = 0;
        }
    }
    pub fn move_zero(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }
            fast += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_283() {
        let mut vec = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec![1, 3, 12, 0, 0], vec);
        vec = vec![0];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec![0], vec);
    }
}
