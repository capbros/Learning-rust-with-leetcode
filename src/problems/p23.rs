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

#[derive(PartialEq, Eq, Clone, Debug)]
struct HeapNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let heads = lists.into_iter().filter_map(|x| x).collect::<Vec<_>>();
        let mut heap =
            std::collections::BinaryHeap::from_iter(heads.into_iter().map(|node| HeapNode {
                val: node.val,
                next: node.next,
            }));
        let mut merged_list = None;
        let mut current = &mut merged_list;

        while let Some(HeapNode { val, next }) = heap.pop() {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
            if let Some(node) = next {
                heap.push(HeapNode {
                    val: node.val,
                    next: node.next,
                });
            }
        }

        merged_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    #[test]
    fn example1() {
        let lists = vec![
            list_from_vec(vec![1, 4, 5]),
            list_from_vec(vec![1, 3, 4]),
            list_from_vec(vec![2, 6]),
        ];
        let merged = Solution::merge_k_lists(lists);
        let expected = list_from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(merged, expected);
    }

    #[test]
    fn example2() {
        let lists = vec![];
        let merged = Solution::merge_k_lists(lists);
        let expected = None;
        assert_eq!(merged, expected);
    }
}
