pub struct Solution;

// static DIG_MAP: [Vec<char>; 8] = {
//     vec!['a', 'b', 'c']
// }

impl Solution {
    fn digit_to_len(digit: u32) -> usize {
        match digit {
            7 | 9 => 4,
            _ => 3,
        }
    }

    fn digit_to_char(d: u32, off: u32) -> char {
        let base;
        if d >= 9 {
            base = 'w' as u32;
        }
        else if d >= 7 {
            base = ('p' as u32) + (d - 7) * 4;
        }
        else {
            base = ('a' as u32) + (d - 2) * 3;
        }
        char::from_u32(base + off).expect("Failed to produce char")
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits
            .chars()
            .map(|d| d.to_digit(10).expect("Non digit found in input"));
        let total_combs: usize = digits.clone().fold(1, |r, d| r * Self::digit_to_len(d));
        let mut res = vec![String::new(); total_combs];

        let mut zone_size = total_combs;
        for d in digits {
            let d_len = Self::digit_to_len(d) as u32;
            zone_size /= Self::digit_to_len(d);
            let mut c_off: u32 = 0;
            let mut c = Self::digit_to_char(d, c_off);
            for (i, s) in res.iter_mut().enumerate() {
                s.push(c);
                if (i + 1) % zone_size == 0 {
                    c_off = (c_off + 1) % d_len;
                    c = Self::digit_to_char(d, c_off);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(input: &str, exp: &[&str]) {
        let exp: Vec<String> = exp.iter().map(|s| s.to_string()).collect();
        let input = input.to_string();
        assert_eq!(exp, Solution::letter_combinations(input));
    }

    #[test]
    fn example1() {
        run_test(
            "23",
            &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
        );
    }

    #[test]
    fn example2() {
        run_test("2", &["a", "b", "c"]);
    }

    #[test]
    fn test16() {
        run_test("7", &["p", "q", "r", "s"]);
    }
}
