use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        const BUDGET: i32 = 8;

        let mut occurrences = [0; 26];
        for c in word.bytes() {
            occurrences[(c - b'a') as usize] += 1;
        }

        let mut heap = BinaryHeap::from(occurrences);

        let mut res = 0;
        let mut iter = 0;
        while heap.peek().is_some_and(|&x| x > 0) {
            let cur_occ = heap.pop().unwrap();
            res += cur_occ * ((iter / BUDGET) + 1);
            iter += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_string()));
    }

    #[test]
    fn example2() {
        assert_eq!(12, Solution::minimum_pushes("xycdefghij".to_string()));
    }
}