pub struct Solution;

impl Solution {
    fn is_open(c: u8) -> bool {
        [b'(', b'[', b'{'].contains(&c)
    }

    fn close_matches(c: u8, open: u8) -> bool {
        let matching = match c {
            b')' => b'(',
            b']' => b'[',
            b'}' => b'{',
            _ => return false,
        };
        matching == open
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<_> = Vec::new();
        for c in s.bytes() {
            if Solution::is_open(c) {
                stack.push(c);
            } else {
                if let Some(open) = stack.last() {
                    if !Solution::close_matches(c, *open) {
                        return false;
                    }
                    stack.pop();
                }
                else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn example2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn example4() {
        assert!(Solution::is_valid("([])".to_string()));
    }

    #[test]
    fn example5() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }
}
