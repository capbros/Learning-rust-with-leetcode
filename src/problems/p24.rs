pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head.as_deref();
        let mut res: Option<Box<ListNode>> = None;
        let mut target = &mut res;
        while let Some(cur) = p {
            if let Some(next) = cur.next.as_deref() {
                let node = target.insert(Box::new(ListNode::new(next.val)));
                target = &mut node.next;
                p = next.next.as_deref();
            }
            else {
                p = cur.next.as_deref();
            }
            let node = target.insert(Box::new(ListNode::new(cur.val)));
            target = &mut node.next;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    fn build_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut p = &mut res;

        for val in v {
            let node = p.insert(Box::new(ListNode::new(val)));
            p = &mut node.next;
        }

        res
    }

    fn list_to_vec(v: Option<Box<ListNode>>) -> Vec<i32> {
        let mut p = v.as_deref();
        let mut res = Vec::new();

        while let Some(cur) = p {
            res.push(cur.val);
            p = cur.next.as_deref();
        }

        res
    }

    #[test]
    fn example1() {
        assert_eq!(
            [2, 1, 4, 3].to_vec(),
            list_to_vec(Solution::swap_pairs(build_list([1, 2, 3, 4].to_vec())))
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Vec::<i32>::new(),
            list_to_vec(Solution::swap_pairs(build_list(Vec::new())))
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            [1].to_vec(),
            list_to_vec(Solution::swap_pairs(build_list([1].to_vec())))
        )
    }

    #[test]
    fn example4() {
        assert_eq!(
            [2, 1, 3].to_vec(),
            list_to_vec(Solution::swap_pairs(build_list([1, 2, 3].to_vec())))
        )
    }
}
