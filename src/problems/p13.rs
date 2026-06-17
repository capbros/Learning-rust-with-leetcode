/**
 * Problem 13. Roman to Integer
 */

pub struct Solution;

impl Solution {
    #[inline]
    pub fn char_to_int(c: u8) -> i32 {
        match c {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => -1, 
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut last = 1000;
        let mut res = 0;
        for c in s.bytes() {
            let cur = Self::char_to_int(c);
            if cur > last {
                res += cur - 2 * last;
            }
            else {
                res += cur;
            }
            last = cur;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    
    fn apply(literal: &str) -> i32 {
        Solution::roman_to_int(String::from(literal))
    }

    #[test]
    fn example1() {
        assert_eq!(3, apply("III"))
    }

    #[test]
    fn example2() {
        assert_eq!(58, apply("LVIII"));
    }

    #[test]
    fn example3() {
        assert_eq!(1994, apply("MCMXCIV"));
    }
}