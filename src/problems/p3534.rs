use std::vec;

pub struct Solution;

// From utils::int::get_msb
pub fn get_msb(mut n: usize) -> u32 {
    let mut i: u32 = 0;
    while n > 0 {
        n >>= 1;
        i += 1;
    }
    i
}

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // Number of nodes in the graph
        let n = n as usize;
        // Sorted indexes of the nums array (i < j <=> nums[idx[i]] < nums[idx[j]])
        let idx: Vec<usize> = {
            let mut node_range: Vec<usize> = (0..n).collect();
            node_range.sort_by(|&a, &b| nums[a].cmp(&nums[b]));
            node_range
        };
        // Reverse index mapping (pos[i] = p <=> idx[p] = i)
        let pos: Vec<usize> = {
            let mut pos: Vec<_> = vec![0;n];
            for (i, &p) in idx.iter().enumerate() {
                pos[p] = i;
            }
            pos
        };
        // Vector containing for each sorted node index a range of sorted node indexes
        // that are connected to it
        let ranges = {
            let mut left = 0;
            let mut right = 0;
            let mut ranges = vec![[0, 0]; n];
            while left < n {
                if right < n && nums[idx[right]] - nums[idx[left]] <= max_diff {
                    ranges[right][0] = left;
                    right += 1;
                }
                else {
                    ranges[left][1] = right - 1;
                    left += 1;
                }
            }
            ranges
        };
        
        let missing = n;
        let m = get_msb(n) as usize;
        
        let mut jump_map: Vec<Vec<_>> = ranges
            .iter()
            .map(|r| {
                let mut v = vec![missing; m];
                v[0] = r[0];
                v
            })
            .collect();

        for j in 1..m {
            for i in 0..n {
                jump_map[i][j] = jump_map[jump_map[i][j-1]][j-1];
            }
        }

        let mut res: Vec<_> = Vec::new();
        res.reserve(queries.len());
        for v in queries {
            let (mut x, mut y) = (pos[v[0] as usize], pos[v[1] as usize]);
            if x == y {
                res.push(0);
                continue;
            }
            if x > y {
                std::mem::swap(&mut x, &mut y);
            }
            let mut step = 0;
            for i in (0..m).rev() {
                if jump_map[y][i] > x {
                    step += 1 << i;
                    y = jump_map[y][i];
                }
            }

            res.push(if jump_map[y][0] <= x { step + 1 } else { -1 });
        }

        res
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
