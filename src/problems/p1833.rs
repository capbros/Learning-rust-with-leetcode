pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for n in costs {
            heap.push(std::cmp::Reverse(n));
        }
        let mut res = 0;
        while let Some(Reverse(el)) = heap.pop()
            && coins - el >= 0
        {
            res += 1;
            coins -= el;
        }
        res
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    fn run_test(costs: &[i32], coins: i32, exp: i32) {
        assert_eq!(exp, Solution::max_ice_cream(costs.to_vec(), coins));
    }

    #[test]
    fn example1() {
        run_test(&[1, 3, 2, 4, 1], 7, 4);
    }

    #[test]
    fn example2() {
        run_test(&[10, 6, 8, 7, 7, 8], 5, 0);
    }

    #[test]
    fn example3() {
        run_test(&[1, 6, 3, 1, 2, 5], 20, 6);
    }
}
