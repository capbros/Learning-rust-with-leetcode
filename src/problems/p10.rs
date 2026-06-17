pub struct Solution;

impl Solution {
    fn match_recursive(s: &[u8], p: &[u8], si: usize, pi: usize, table: &mut Vec<Vec<bool>>) -> bool{
        let mut si_copy = si;
        let mut pi_copy = pi;
        while pi_copy < p.len() && si_copy < s.len() {
            if table[si_copy][pi_copy] {
                return false;
            }
            let sc = s[si_copy];
            let pc = p[pi_copy];
            if pi_copy < p.len() - 1 && p[pi_copy + 1] == b'*' {
                if pc == b'.' || pc == sc {
                    let res = Self::match_recursive(s, p, si_copy + 1, pi_copy, table);
                    if si_copy < s.len() - 1 {
                        table[si_copy + 1][pi_copy] = true;
                    }
                    if res {
                        return true;
                    }
                }
                pi_copy += 2;
            }
            else if pc != b'.' && pc != sc {
                return false;
            }
            else {
                pi_copy += 1;
                si_copy += 1;
            }
        }
        while pi_copy < p.len() - 1 && p[pi_copy + 1] == b'*' {
            pi_copy += 2;
        }
        if pi_copy >= p.len() && si_copy >= s.len() {
            return true;
        }
        false
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut table = vec!{vec!{false; p.len()}; s.len()};
        return Self::match_recursive(s.as_bytes(), p.as_bytes(), 0, 0, &mut table);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(s: &str, p: &str, exp: bool) {
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(exp, res);
    }

    #[test]
    fn example1() {
        run_test("aa", "a", false);
    }

    #[test]
    fn example2() {
        run_test("aa", "a*", true);
    }

    #[test]
    fn example3() {
        run_test("ab", ".*", true);
    }

    #[test]
    fn test22() {
        run_test("mississippi", "mis*is*ip*.", true);
    }

    #[test]
    fn test284() {
        run_test("aab", "c*a*b", true);
    }

    #[test]
    fn test320() {
        run_test("mississippi", "mis*is*p*.", false);
    }
}
