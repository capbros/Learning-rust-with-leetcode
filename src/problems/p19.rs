pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_ref();
        }

        let mut link = &mut head;
        for _ in 0..(len - n as usize) {
            link = &mut link.as_mut().expect("linked list shorter than n").next;
        }

        let mut removed = link.take().expect("node to remove must exist");
        *link = removed.next.take();

        head
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    fn list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &value in values.iter().rev() {
            head = Some(Box::new(ListNode {
                val: value,
                next: head,
            }));
        }
        head
    }

    fn values(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::new();
        while let Some(node) = head {
            res.push(node.val);
            head = node.next;
        }
        res
    }

    #[test]
    fn example1() {
        let head = list(&[1, 2, 3, 4, 5]);
        let res = Solution::remove_nth_from_end(head, 2);
        assert_eq!(values(res), vec![1, 2, 3, 5]);
    }

    #[test]
    fn example2() {
        let head = list(&[1]);
        let res = Solution::remove_nth_from_end(head, 1);
        assert_eq!(values(res), vec![]);
    }

    #[test]
    fn example3() {
        let head = list(&[1, 2]);
        let res = Solution::remove_nth_from_end(head, 1);
        assert_eq!(values(res), vec![1]);
    }
}
