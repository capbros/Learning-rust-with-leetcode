/**
 * Problem 12. Integer to Roman
 */
pub struct Solution;

struct RomanChars {
    one: char, five: char
}

static CHAR_MAP: [RomanChars; 4] = [
    RomanChars{
        one: 'I',
        five: 'V',
    },
    RomanChars{
        one: 'X',
        five: 'L',
    },
    RomanChars{
        one: 'C',
        five: 'D',
    },
    RomanChars{
        one: 'M',
        five: '#',
    }
];

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut res = String::new();
        let mut digits = 0;
        let mut num_rev = 0;
        while num > 0 {
            num_rev *= 10;
            num_rev += num % 10;
            num /= 10;
            digits += 1;
        }
        while num_rev > 0 {
            digits -= 1;
            let d = num_rev % 10;
            num_rev /= 10;
            let chars = &CHAR_MAP[digits];
            if d < 4 {
                res.extend(std::iter::repeat(chars.one).take(d as usize));
            }
            else if d == 4 {
                res.push(chars.one);
                res.push(chars.five);
            }
            else if d >= 5 && d < 9 {
                res.push(chars.five);
                res.extend(std::iter::repeat(chars.one).take((d - 5) as usize));
            }
            else {
                let ten = CHAR_MAP[digits + 1].one;
                res.push(chars.one);
                res.push(ten);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_test(num: i32, exp: &str) {
        let res = Solution::int_to_roman(num);
        assert_eq!(exp, res.as_str());
    }

    #[test]
    fn example1() {
        run_test(3749, "MMMDCCXLIX");
    }

    #[test]
    fn example2() {
        run_test(58, "LVIII");
    }

    #[test]
    fn example3() {
        run_test(1994, "MCMXCIV");
    }
}