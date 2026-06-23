pub struct Solution;

struct CompressedElement<T: PartialEq + Clone> {
    el: T,
    count: u16,
}

struct CompressedVec<T: PartialEq + Clone> {
    vec: Vec<CompressedElement<T>>,
}

impl<T: PartialEq + Clone> CompressedVec<T> {
    fn new() -> CompressedVec<T> {
        CompressedVec { vec: Vec::new() }
    }

    fn push(&mut self, el: T) {
        if let Some(top) = self.vec.last_mut()
            && top.el == el
        {
            top.count += 1;
        } else {
            self.vec.push(CompressedElement { el, count: 1 });
        }
    }

    fn last(&self) -> Option<&T> {
        self.vec.last().map(|el| &el.el)
    }

    fn pop(&mut self) -> Option<T> {
        let el = self.vec.last_mut()?;
        el.count -= 1;
        if el.count == 0 {
            return Some(self.vec.pop()?.el);
        }
        Some(el.el.clone())
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

impl Solution {
    fn is_open(c: u8) -> bool {
        [b'(', b'[', b'{'].contains(&c)
    }

    fn close_matches(c: u8, open: u8) -> bool {
        let matching = match c {
            b')' => b'(',
            b']' => b'[',
            b'}' => b'{',
            _ => return false,
        };
        matching == open
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack: CompressedVec<u8> = CompressedVec::new();
        for c in s.bytes() {
            if Solution::is_open(c) {
                stack.push(c);
            } else {
                if let Some(open) = stack.last() {
                    if !Solution::close_matches(c, *open) {
                        return false;
                    }
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn example2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn example4() {
        assert!(Solution::is_valid("([])".to_string()));
    }

    #[test]
    fn example5() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }
}
