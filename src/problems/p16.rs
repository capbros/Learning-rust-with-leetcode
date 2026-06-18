/**
 * Problem 16. 3Sum closest
 */
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();

        nums.sort_unstable();
        let mut cur_res = nums[0] + nums[1] + nums[2];
        let mut min_diff = (target - cur_res).abs();

        for i in 0..n - 2 {
            // Skip duplicate values for i
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            // If the min sum, with i fixed, is above target, going further than this point is useless
            {
                let min_sum = nums[i] + nums[i + 1] + nums[i + 2];
                if min_sum > target {
                    let cur_diff = (target - min_sum).abs();
                    if cur_diff < min_diff {
                        cur_res = min_sum;
                    }
                    break;
                }
            }

            let mut j = i + 1;
            let mut k = n - 1;

            // If the max sum with i fixed is below target, there is no need
            // to check other triplets
            {
                let max_sum = nums[i] + nums[k - 1] + nums[k];
                if max_sum < target {
                    let cur_diff = (target - max_sum).abs();
                    if cur_diff < min_diff {
                        cur_res = max_sum;
                        min_diff = cur_diff;
                    }
                    continue;
                }
            }

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return target;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
                let diff = (target - sum).abs();
                if diff < min_diff {
                    cur_res = sum;
                    min_diff = diff;
                }
            }
        }
        cur_res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(nums: &[i32], target: i32, exp: i32) {
        let nums = nums.to_vec();
        assert_eq!(exp, Solution::three_sum_closest(nums, target));
    }

    #[test]
    fn example1() {
        run_test(&[-1, 2, 1, -4], 1, 2);
    }

    #[test]
    fn example2() {
        run_test(&[0, 0, 0], 1, 0);
    }

    #[test]
    fn test109() {
        run_test(&[2, 5, 6, 7], 16, 15);
    }
}
