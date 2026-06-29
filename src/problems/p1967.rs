pub struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut res = 0;
        for p in patterns {
            if word.find(&p).is_some() {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

}