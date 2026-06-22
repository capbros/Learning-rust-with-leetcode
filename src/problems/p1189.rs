pub struct Solution;

impl Solution {
    fn char_to_index(c: char) -> Option<usize> {
        let index = match c {
            'b' => 0,
            'a' => 1,
            'l' => 2,
            'o' => 3,
            'n' => 4,
            _ => return None,
        };
        Some(index)
    }

    fn occurrences(index: usize) -> i32 {
        match index {
            2 | 3 => 2,
            _ => 1,
        }
    }

    pub fn max_number_of_balloons(text: String) -> i32 {
        const NUM_CHARS: usize = 5;
        let mut occurrences = [0; NUM_CHARS];
        for c in text.chars() {
            if let Some(index) = Solution::char_to_index(c) {
                occurrences[index] += 1;
            }
        }
        occurrences
            .iter()
            .enumerate()
            .map(|(index, count)| count / Self::occurrences(index))
            .min()
            .expect("Empty occurrences array")
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_string()));
    }

    #[test]
    fn example2() {
        assert_eq!(
            2,
            Solution::max_number_of_balloons("loonbalxballpoon".to_string())
        );
    }

    #[test]
    fn example3() {
        assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_string()));
    }
}
