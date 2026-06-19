pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter().fold((0, 0), |res, next| {
            let cur = res.1 + *next;
            (if cur > res.0 {cur} else {res.0}, cur)
        }).0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(gain: &[i32], exp: i32) {
        assert_eq!(exp, Solution::largest_altitude(gain.to_vec()));
    }

    #[test]
    fn example1() {
        run_test(&[-5,1,5,0,-7], 1);
    }

    #[test]
    fn example2() {
        run_test(&[-4,-3,-2,-1,4,3,2], 0);
    }
}