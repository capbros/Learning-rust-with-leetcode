pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let n = nums.len();
        if n < 4 {
            return Vec::new();
        }

        nums.sort_unstable();
        let mut res = Vec::new();

        for a in 0..n - 3 {
            // Skip duplicate values for a
            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }

            {
                let max_sum =
                    nums[a] as i64 + nums[n - 1] as i64 + nums[n - 2] as i64 + nums[n - 3] as i64;
                if max_sum < target {
                    continue;
                }
            }

            for b in a + 1..n - 2 {
                // Skip duplicate values for b
                if b > a + 1 && nums[b] == nums[b - 1] {
                    continue;
                }

                {
                    let min_sum =
                        nums[a] as i64 + nums[b] as i64 + nums[b + 1] as i64 + nums[b + 2] as i64;
                    if min_sum > target {
                        break;
                    }
                }

                {
                    let max_sum =
                        nums[a] as i64 + nums[b] as i64 + nums[n - 1] as i64 + nums[n - 2] as i64;
                    if max_sum < target {
                        continue;
                    }
                }

                let mut c = b + 1;
                let mut d = n - 1;
                while c < d {
                    let sum = nums[a] as i64 + nums[b] as i64 + nums[c] as i64 + nums[d] as i64;
                    if sum == target {
                        res.push(vec![nums[a], nums[b], nums[c], nums[d]]);

                        // Skip duplicates for j and k
                        while c < d && nums[c] == nums[c + 1] {
                            c += 1;
                        }
                        while c < d && nums[d] == nums[d - 1] {
                            d -= 1;
                        }
                        c += 1;
                        d -= 1;
                    } else if sum < target {
                        c += 1;
                    } else {
                        d -= 1;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(nums: &[i32], target: i32, exp: &[[i32; 4]]) {
        let nums = nums.to_vec();
        let exp: Vec<Vec<i32>> = exp.iter().map(|x| x.to_vec()).collect();
        let res = Solution::four_sum(nums, target);
        assert_eq!(exp, res);
    }

    #[test]
    fn exaple1() {
        run_test(
            &[1, 0, -1, 0, -2, 2],
            0,
            &[[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]],
        );
    }

    #[test]
    fn exaple2() {
        run_test(&[2, 2, 2, 2, 2], 8, &[[2, 2, 2, 2]]);
    }

    #[test]
    fn test293() {
        run_test(
            &[1000000000, 1000000000, 1000000000, 1000000000],
            -294967296,
            &[],
        );
    }
}
