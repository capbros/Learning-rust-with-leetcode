/**
 * Problem 14. Longest Common Prefix
 */
pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let Some(first) = strs.first() else {
            return String::new();
        };

        let first_bytes = first.bytes();
        let mut end = first_bytes.len();

        for s in &strs[1..] {
            let first_copy = first_bytes.clone();
            let bytes = s.bytes();
            let mut i = 0;
            for (c, c_ref) in std::iter::zip(bytes, first_copy) {
                if i >= end || c != c_ref {
                    break;
                }
                i += 1;
            }
            end = i;
            if end == 0 {
                break;
            }
        }

        first[..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(strs: &[&str], exp: &str) {
        let input_strs: Vec<_> = strs.iter().map(|s| s.to_string()).collect();
        let res = Solution::longest_common_prefix(input_strs);
        assert_eq!(exp.to_string(), res);
    }

    #[test]
    fn example1() {
        run_test(&["flower", "flow", "flight"], "fl");
    }

    #[test]
    fn example2() {
        run_test(&["dog", "racecar", "car"], "");
    }
}
