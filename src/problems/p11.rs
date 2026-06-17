/**
 * Problem 11. Container With Most Water
 */
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut iterator = height.iter();
        let mut base = (height.len() - 1) as i32;
        let (mut left, mut right) =
            if let (Some(l), Some(r)) = (iterator.next(), iterator.next_back()) {
                (l, r)
            } else {
                return 0;
            };
        let mut end = false;
        let mut res = 0;
        while !end {
            let cur: i32;
            if *left > *right {
                cur = *right * base;
                if let Some(r) = iterator.next_back() {
                    right = r;
                }
                else {
                    end = true;
                }
            } else {
                cur = *left * base;
                if let Some(l) = iterator.next() {
                    left = l;
                }
                else {
                    end = true;
                }
            }
            if cur > res {
                res = cur;
            }
            base -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_example(height: &[i32], exp: i32) {
        let res = Solution::max_area(height.to_vec());
        assert_eq!(exp, res);
    }

    #[test]
    fn example1() {
        run_example(&[1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
    }

    #[test]
    fn example2() {
        run_example(&[1, 1], 1);
    }
}
