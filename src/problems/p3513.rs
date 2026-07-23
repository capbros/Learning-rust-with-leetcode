pub struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut unique = 1;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        while n > 0 {
            n >>= 1;
            unique <<= 1;
        }
        return unique;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
       assert_eq!(2, Solution::unique_xor_triplets([1, 2].to_vec()));
    }

    #[test]
    fn example2() {
        assert_eq!(4, Solution::unique_xor_triplets([3, 1, 2].to_vec()));
    }
}