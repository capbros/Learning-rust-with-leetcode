/**
 * Problem 15. 3Sum
 */
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        // Size check is unnecessary
        // if n < 3 {
        //     return Vec::new();
        // }
        
        nums.sort_unstable();
        let mut res = Vec::new();

        for i in 0..n - 2 {
            // Skip duplicate values for i
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            // Optimization: If current smallest is > 0, sum can never be 0
            if nums[i] > 0 {
                break;
            }

            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    
                    // Skip duplicates for j and k
                    while j < k && nums[j] == nums[j + 1] { j += 1; }
                    while j < k && nums[k] == nums[k - 1] { k -= 1; }
                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(nums: &[i32], exp: &[&[i32]]) {
        let exp: Vec<Vec<i32>> = exp.iter().map(|e| e.to_vec()).collect();
        let nums = nums.to_vec();
        assert_eq!(exp, Solution::three_sum(nums));
    }

    #[test]
    fn example1() {
        run_test(&[-1, 0, 1, 2, -1, -4], &[&[-1, -1, 2], &[-1, 0, 1]]);
    }

    #[test]
    fn example2() {
        run_test(&[0, 1, 1], &[]);
    }

    #[test]
    fn example3() {
        run_test(&[0, 0, 0], &[&[0, 0, 0]]);
    }
}
