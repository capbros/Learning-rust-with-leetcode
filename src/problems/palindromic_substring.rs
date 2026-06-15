use std::cmp::min;

/**
 * Problem 5. Longest Palindromic Substring
 */

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res = &s[0..0];
        let s_bytes = s.as_bytes();
        for i in 0..s.len() {
            let checks = min(i, s.len() - i - 1);
            let mut half_size = 0;
            if 2 * checks + 1 <= res.len() {
                continue;
            }
            while half_size < checks {
                if s_bytes[i - 1 - half_size] != s_bytes[i + 1 + half_size] {
                    break;
                }
                half_size += 1;
            }
            if 2 * half_size + 1 > res.len() {
                res = &s[i - half_size..=i + half_size];
            }
        }
        for i in 0..s.len() - 1 {
            let bottom = i;
            let top = i + 1;
            let checks = min(i + 1, s.len() - i - 1);
            let mut half_size = 0;
            if 2 * checks <= res.len() {
                continue;
            }
            while half_size < checks {
                if s_bytes[bottom - half_size] != s_bytes[top + half_size] {
                    break;
                }
                half_size += 1;
            }
            if 2 * half_size > res.len() {
                res = &s[top - half_size..top + half_size];
            }
        }
        String::from(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn is_palindrome(x: String) -> bool {
        x.bytes().eq(x.bytes().rev())
    }

    fn run_test(input: &str, len: usize) {
        let res = Solution::longest_palindrome(String::from(input));
        println!("Output result: {res}");
        assert_eq!(len, res.len());
        assert!(is_palindrome(res));
    }

    #[test]
    fn example1() {
        run_test("babad", 3);
    }

    #[test]
    fn example2() {
        run_test("cbbd", 2);
    }
}