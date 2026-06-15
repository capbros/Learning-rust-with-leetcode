/**
 * Problem 9. Palindrome Number
 */

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        let first_half = &x_str[..x_str.len()/2];
        let second_half = &x_str[(x_str.len() + 1)/2..];
        return first_half.chars().eq(second_half.chars().rev());
    }
}

#[cfg(test)]
mod palindrome_tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn example2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn custom1() {
        assert!(Solution::is_palindrome(1234321));
    }

    #[test]
    fn custom2() {
        assert!(!Solution::is_palindrome(1234421));
    }
}