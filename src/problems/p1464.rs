pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut vi, mut vj) = (0, 0);
        for nval in nums {
            if nval > vi {
                vj = vi;
                vi = nval;
            } else if nval > vj {
                vj = nval;
            }
        }
        (vi - 1) * (vj - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(12, Solution::max_product([3, 4, 5, 2].to_vec()));
    }

    #[test]
    fn example2() {
        assert_eq!(16, Solution::max_product([1, 5, 4, 5].to_vec()));
    }

    #[test]
    fn example3() {
        assert_eq!(12, Solution::max_product([3, 7].to_vec()));
    }
}
