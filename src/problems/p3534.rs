use std::vec;

pub struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let sorted_nums = {
            let mut sorted_nums = nums.iter()
            .enumerate()
            .map(|(i, &num)| (i, num))
            .collect::<Vec<_>>();
            sorted_nums.sort_by(|(_, n), (_, m)| n.cmp(m));
            sorted_nums
        };
        let ranges = {
            let mut left = 0;
            let mut right = 0;
            let mut ranges = vec![vec![0, 0]; n];
            while left < n {
                if right < n && sorted_nums[right].1 - sorted_nums[left].1 <= max_diff {
                    ranges[sorted_nums[right].0][0] = left;
                    right += 1;
                }
                else {
                    ranges[sorted_nums[left].0][1] = right - 1;
                    left += 1;
                }
            }
            ranges
        };

        let results = queries
            .into_iter()
            .map(|query| {
                let (start, end) = (query[0] as usize, query[1] as usize);
                let mut visited = vec![false; n];
                let mut queue = std::collections::VecDeque::from(vec![(start, 0)]);
                while let Some((node, dist)) = queue.pop_front() {
                    if node == end {
                        return dist;
                    }
                    if !visited[node] {
                        visited[node] = true;
                        for neighbor in ranges[node][0]..=ranges[node][1] {
                            let neighbor = sorted_nums[neighbor].0;
                            if !visited[neighbor]
                                && (nums[node] - nums[neighbor]).abs() <= max_diff
                            {
                                queue.push_back((neighbor, dist + 1));
                            }
                        }
                    }
                }
                -1
            })
            .collect();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 5;
        let nums = vec![1, 8, 3, 4, 2];
        let max_diff = 3;
        let queries = vec![vec![0, 3], vec![2, 4]];
        let expected = vec![1, 1];
        assert_eq!(
            Solution::path_existence_queries(n, nums, max_diff, queries),
            expected
        );
    }

    #[test]
    fn example2() {
        let n = 5;
        let nums = vec![5, 3, 1, 9, 10];
        let max_diff = 2;
        let queries = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]];
        let expected = vec![1, 2, -1, 1];
        assert_eq!(
            Solution::path_existence_queries(n, nums, max_diff, queries),
            expected
        );
    }

    #[test  ]
    fn example3() {
        let n = 3;
        let nums = vec![3, 6, 1];
        let max_diff = 1;
        let queries = vec![vec![0, 0], vec![0, 1], vec![1, 2]];
        let expected = vec![0, -1, -1];
        assert_eq!(
            Solution::path_existence_queries(n, nums, max_diff, queries),
            expected
        );
    }
}
