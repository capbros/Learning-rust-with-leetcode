pub struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut counts = vec![0; len];
        for n in arr {
            let n = n as usize;
            if n <= len {
                counts[n - 1] += 1;
            }
            else {
                counts[len - 1] += 1;
            }
        }
        let mut res = 0;
        let mut index = 0;
        while index < len && res <= len {
            if counts[index] == 0 {
                index += 1;
            }
            else {
                counts[index] -= 1;
                res = std::cmp::min(index + 1, res + 1);
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            2,
            Solution::maximum_element_after_decrementing_and_rearranging([2, 2, 1, 2, 1].to_vec())
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            3,
            Solution::maximum_element_after_decrementing_and_rearranging([100, 1, 1000].to_vec())
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            5,
            Solution::maximum_element_after_decrementing_and_rearranging([1, 2, 3, 4, 5].to_vec())
        );
    }

    #[test]
    fn test49() {
        let input = {
            let mut v = [209; 9999].to_vec();
            v.push(10000);
            v
        };
        assert_eq!(
            210,
            Solution::maximum_element_after_decrementing_and_rearranging(input)
        )
    }
}
