pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() {
            if left > 0 && nums[right] == nums[left-1] {
                right += 1;
            } else {
                nums[left] = nums[right];
                left += 1;
                right += 1;
            }
        }
        // nums.truncate(left);
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2].to_vec(), vec![1, 2]);
    }

    #[test]
    fn test_remove_duplicates2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5].to_vec(), vec![0, 1, 2, 3, 4]);
    }
}