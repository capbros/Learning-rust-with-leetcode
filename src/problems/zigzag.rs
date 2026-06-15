/**
 * Problem 6. Zigzag Conversion
 */

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let s_chars = s.as_bytes();
        let mut res = String::new();
        let period: usize = ((num_rows - 1) * 2) as usize;
        let period = if period > 0 { period } else { 1 };

        let mut index;
        for row in 0..num_rows {
            index = row as usize;
            let diff = (period - 2 * index) % period;
            while index < s.len() {
                res.push(s_chars[index] as char);
                if diff > 0 && index + diff < s.len() {
                    res.push(s_chars[index + diff] as char);
                }
                index += period;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(input: &str, num_rows: i32, exp: &str) {
        let res = Solution::convert(String::from(input), num_rows);
        assert_eq!(exp, res);
    }

    #[test]
    fn example1() {
        run_test("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
    }

    #[test]
    fn example2() {
        run_test("PAYPALISHIRING", 4, "PINALSIGYAHRPI");
    }
}
