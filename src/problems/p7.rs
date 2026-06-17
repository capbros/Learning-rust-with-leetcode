pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res = 0;
        let max = if x > 0 { i32::MAX } else { i32::MIN };
        while x != 0 {
            let rem = x % 10;
            x /= 10;
            let overflow = ((res < -1 || res > 1)  && max / res < 10) ||
                if max > 0 {max - res * 10 < rem} else {max - res * 10 > rem};
            if overflow {
                return 0;
            }
            res = res * 10 + rem;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn example2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn example3() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn example14() {
        assert_eq!(-109, Solution::reverse(-901000));
    }
}