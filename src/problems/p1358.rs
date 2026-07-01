pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let (mut a_count, mut b_count, mut c_count) = (0, 0, 0);
        let mut result = 0;

        let mut left = 0;
        let mut right = 0;
        
        let s_bytes = s.as_bytes();
        match s_bytes[0] {
            b'a' => a_count += 1,
            b'b' => b_count += 1,
            b'c' => c_count += 1,
            _ => {}
        }
        while right < s_bytes.len() {
            if a_count > 0 && b_count > 0 && c_count > 0 {
                result += (s_bytes.len() - right) as i32;
                match s_bytes[left] {
                    b'a' => a_count -= 1,
                    b'b' => b_count -= 1,
                    b'c' => c_count -= 1,
                    _ => {}
                }
                left += 1;
            }
            else {
                right += 1;
                if right < s_bytes.len() {
                    match s_bytes[right] {
                        b'a' => a_count += 1,
                        b'b' => b_count += 1,
                        b'c' => c_count += 1,
                        _ => {}
                    }
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::number_of_substrings("abcabc".into()), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::number_of_substrings("aaacb".into()), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::number_of_substrings("abc".into()), 1);
    }
}