/**
 * Problem 8. String to Integer (atoi)
 */

pub struct Solution;

impl Solution {
    fn get_signed(x: i64, negative: bool) -> i64 {
        if negative { -x } else { x }
    }

    pub fn my_atoi(s: String) -> i32 {
        let mut negative = false;
        let mut res: i64 = 0;
        let mut preamble = true;
        for c in s.chars() {
            match c {
                ' ' if preamble => {},  // Ignore leading zeros
                '-' if preamble => {
                    negative = true;
                    preamble = false;  // Adapt sign
                },
                '+' if preamble => {
                    negative = false;  // Adapt sign
                    preamble = false;
                },
                _ => {
                    if let Some(dig) = c.to_digit(10) {
                        res = res * 10 + (dig as i64);
                        preamble = false;
                        let signed_res = Self::get_signed(res, negative);
                        if signed_res < i32::MIN as i64 {
                            return i32::MIN;
                        }
                        else if signed_res > i32::MAX as i64 {
                            return i32::MAX;
                        }
                    }
                    else {
                        break;
                    }
                }
            }
        }
        Self::get_signed(res, negative) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(input: &str, expected: i32) {
        let res = Solution::my_atoi(input.to_string());
        assert_eq!(expected, res);
    }

    #[test]
    fn example1() {
        run_test("42", 42);
    }

    #[test]
    fn example2() {
        run_test("-042", -42);
    }

    #[test]
    fn example3() {
        run_test("1337c0d3", 1337);
    }

    #[test]
    fn example4() {
        run_test("0-1", 0);
    }

    #[test]
    fn example5() {
        run_test("words and 987", 0);
    }

    #[test]
    fn test1088() {
        run_test("+-12", 0);
    }
}