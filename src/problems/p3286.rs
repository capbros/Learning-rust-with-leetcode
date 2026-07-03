pub struct Solution;

use std::collections::VecDeque;

struct QueueEntry {
    row: usize,
    column: usize,
    health: i32,
}

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let mut deque: VecDeque<QueueEntry> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> =
            vec![vec![false; grid[0].len()]; grid.len()];
        deque.push_back(QueueEntry {
            row: 0,
            column: 0,
            health: if grid[0][0] > 0 {
                health - 1
            } else {
                health
            },
        });
        visited[0][0] = true;

        while let Some(head) = deque.pop_front() {
            if head.row == grid.len() - 1 && head.column == grid[0].len() - 1  && head.health > 0 {
                return true;
            }
            else if head.health <= 0 {
                break;
            }

            let mut enqueue_point = |row: usize, column: usize| {
                if visited[row][column] {
                    return false;
                }
                let new_health;
                if grid[row][column] > 0 {
                    new_health = head.health - 1;
                    deque.push_back(QueueEntry {
                        row,
                        column,
                        health: new_health,
                    });
                } else {
                    new_health = head.health;
                    deque.push_front(QueueEntry {
                        row,
                        column,
                        health: new_health,
                    });
                }
                visited[row][column] = true;
                return row == grid.len() - 1 && column == grid[0].len() - 1 && new_health > 0;
            };
            let mut found = false;
            if head.column > 0 {
                found |= enqueue_point(head.row, head.column - 1);
            }
            if head.column < grid[0].len() - 1 {
                found |= enqueue_point(head.row, head.column + 1);
            }
            if head.row > 0 {
                found |= enqueue_point(head.row - 1, head.column);
            }
            if head.row < grid.len() - 1 {
                found |= enqueue_point(head.row + 1, head.column);
            }
            if found {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_safe_walk() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let health = 1;
        assert_eq!(Solution::find_safe_walk(grid, health), true);
    }

    #[test]
    fn test557() {
        let grid = [[1,1,1,1].to_vec()].to_vec();
        let health = 4;
        assert_eq!(Solution::find_safe_walk(grid, health), false);
    }
}