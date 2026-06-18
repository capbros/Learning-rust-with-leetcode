/**
 * Problem 15. 3Sum
 */
pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut i = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        while i < n - 2 {
            let e1 = nums[i];
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k
            {
                let e2 = nums[j];
                let e3 = nums[k];
                let sum = e1 + e2 + e3;
                if sum == 0 {
                    res.push(vec![e1, e2, e3]);
                }
                if sum >= 0 {
                    loop {
                        k -= 1;
                        if k <= j || nums[k] != e3 {
                            break;
                        }
                    }
                } else {
                    j += 1;
                }
            }
            loop {
                i += 1;
                if i >= n - 2 || nums[i] != e1 {
                    break;
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
