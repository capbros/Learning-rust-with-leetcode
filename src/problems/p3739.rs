pub struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let pref_sums: Vec<_> = nums.iter().scan(0, |state, &cur| {
            if cur == target {
                *state += 1;
            }
            return Some(*state);
        }).collect();
        let mut res: i64 = 0;

        let mut left = 0;
        for i in 0..pref_sums.len() {
            for j in i..pref_sums.len() {
                let half_len = (j - i + 1) / 2;
                if pref_sums[j] - left > half_len {
                    res += 1;
                }
            }
            left = pref_sums[i];
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(5, Solution::count_majority_subarrays([1,2,2,3].to_vec(), 2));
    }

    #[test]
    fn example1ì2() {
        assert_eq!(10, Solution::count_majority_subarrays([1,1,1,1].to_vec(), 1));
    }

    #[test]
    fn example3() {
        assert_eq!(0, Solution::count_majority_subarrays([1,2,3].to_vec(), 4));
    }
}