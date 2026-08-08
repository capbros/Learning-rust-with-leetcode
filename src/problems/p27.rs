pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len();
        while left < right && nums[right - 1] == val {
            right -= 1;
        }
        while left < right {
            if nums[left] == val {
                nums.swap(left, right - 1);
                while left < right && nums[right - 1] == val {
                    right -= 1;
                }
            }
            left += 1;
        }
        right as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use super::Solution;

    #[test]
    fn example1() {
        let mut input = [3, 2, 2, 3].to_vec();
        let res = Solution::remove_element(&mut input, 3);
        assert_eq!(2, res);
        testing::equals_unordered([2, 2].to_vec(), input[..2].to_vec());
    }

    #[test]
    fn example2() {
        let mut input = [0, 1, 2, 2, 3, 0, 4, 2].to_vec();
        let res = Solution::remove_element(&mut input, 2);
        assert_eq!(5, res);
        testing::equals_unordered([0, 1, 4, 0, 3].to_vec(), input[..5].to_vec());
    }
}
