pub struct Solution;

// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = list1.as_ref();
        let mut p2 = list2.as_ref();
        let mut head = None;
        let mut cur: Option<&mut Box<ListNode>> = None;

        loop {
            let nval;
            match (p1, p2) {
                (None, None) => break,
                (Some(el1), Some(el2)) => {
                    if el1.val < el2.val {
                        nval = ListNode::new(el1.val);
                        p1 = el1.next.as_ref();
                    } else {
                        nval = ListNode::new(el2.val);
                        p2 = el2.next.as_ref();
                    }
                }
                (Some(el1), None) => {
                    nval = ListNode::new(el1.val);
                    p1 = el1.next.as_ref();
                }
                (None, Some(el2)) => {
                    nval = ListNode::new(el2.val);
                    p2 = el2.next.as_ref();
                }
            }
            let nnode = Some(Box::from(nval));
            if let Some(cur_node) = cur {
                cur_node.next = nnode;
                cur = cur_node.next.as_mut();
            } else {
                head = nnode;
                cur = head.as_mut();
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut cur: Option<&mut Box<ListNode>> = None;
        for el in v {
            let node = Some(Box::from(ListNode::new(el)));
            if let Some(cur_node) = cur {
                cur_node.next = node;
                cur = cur_node.next.as_mut();
            } else {
                head = node;
                cur = head.as_mut();
            }
        }
        head
    }

    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut cur = list.as_ref();
        let mut res = Vec::new();
        while let Some(node) = cur {
            let (v, next) = (node.val, node.next.as_ref());
            cur = next;
            res.push(v);
        }
        res
    }

    #[test]
    fn example1() {
        assert_eq!(
            [1, 1, 2, 3, 4, 4].to_vec(),
            list_to_vec(Solution::merge_two_lists(
                vec_to_list([1, 2, 4].to_vec()),
                vec_to_list([1, 3, 4].to_vec())
            ))
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            [].to_vec() as Vec<i32>,
            list_to_vec(Solution::merge_two_lists(
                vec_to_list([].to_vec()),
                vec_to_list([].to_vec())
            ))
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            [0].to_vec() as Vec<i32>,
            list_to_vec(Solution::merge_two_lists(
                vec_to_list([].to_vec()),
                vec_to_list([0].to_vec())
            ))
        )
    }
}
