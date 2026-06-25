pub struct Solution;

impl Solution {
    fn generate_parenthesis_rec(n: i32, to_close: i32, mut s: String, res: &mut Vec<String>) -> String {
        if n == 0 && to_close == 0 {
            res.push(s.clone());
        } else {
            if n > 0 {
                s.push('(');
                s = Self::generate_parenthesis_rec(n - 1, to_close + 1, s, res);
                s.pop();
            }
            if to_close > 0 {
                s.push(')');
                s = Self::generate_parenthesis_rec(n, to_close - 1, s, res);
                s.pop();
            }
        }
        s
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        Self::generate_parenthesis_rec(n, 0, "".to_string(), &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::utils::testing;

    #[test]
    fn example1() {
        testing::equals_unordered(
            ["((()))","(()())","(())()","()(())","()()()"].map(|x| x.to_string()).to_vec(),
            Solution::generate_parenthesis(3)
        );
    }

    #[test]
    fn example2() {
        testing::equals_unordered(
            ["()"].map(|x| x.to_string()).to_vec(),
            Solution::generate_parenthesis(1)
        );
    }
}