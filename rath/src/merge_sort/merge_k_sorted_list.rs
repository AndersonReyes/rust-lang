#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl std::cmp::PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl std::cmp::Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // push all the numbers into the heap. O(n) + heap aux space
    // let mut min_heap: std::collections::BinaryHeap<i32> =
    //      std::collections::BinaryHeap::new();
    // TODO: can we do better and not need to push all numbers into heap?
    //for node in lists {
    //    let mut curr = node.clone();

    //    while let Some(c) = curr {
    //        min_heap.push(c.val);
    //        curr = c.next;
    //    }
    //}
    //
    //let curr: &mut Option<Box<ListNode>> = &mut None;

    //while min_heap.len() > 0 {
    //    // unwrap is safe because len is always > 0
    //    let value: i32 = min_heap.pop().unwrap();

    //    if curr.is_some() {
    //        let mut temp = ListNode::new(value);
    //        temp.next = curr.clone();

    //        *curr = Some(Box::new(temp.clone()));
    //    } else {
    //        *curr = Some(Box::new(ListNode::new(value)));
    //    }
    //}

    //curr.clone()
    //

    // Improvement: insert the first nodes into the heap. As we pop the min, insert the
    // min.next in the heap and repeat. We get the min and insert the next item from the list
    // we just popped from.

    let mut heap: std::collections::BinaryHeap<std::cmp::Reverse<Box<ListNode>>> =
        std::collections::BinaryHeap::new();

    for list in lists {
        match list {
            Some(node) => heap.push(std::cmp::Reverse(node)),
            None => {}
        };
    }

    let mut head: Box<ListNode> = Box::new(ListNode::new(0));
    let mut curr: &mut Box<ListNode> = &mut head;

    while heap.len() > 0 {
        let next: Box<ListNode> = heap.pop().unwrap().0;

        curr.next = Some(Box::new(ListNode::new(next.val)));

        if let Some(b) = next.next {
            heap.push(std::cmp::Reverse(b));
        }
        curr = curr.next.as_mut().unwrap();
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        assert_eq!(None, merge_k_lists(vec![]));
    }

    #[test]
    fn test_singleton_list() {
        let l = Some(Box::new(ListNode::new(5)));
        let actual = merge_k_lists(vec![None, l]).unwrap();

        assert_eq!(5, actual.val);
        assert_eq!(None, actual.next);
    }

    #[test]
    fn test_singleton_none_list() {
        let actual = merge_k_lists(vec![None]);

        assert_eq!(None, actual);
    }

    #[test]
    fn test_valid_list() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(5))),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode::new(4))),
        }));

        let list = vec![None, l1, None, l2];
        let actual = merge_k_lists(list);

        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        }));
        assert_eq!(expected, actual);
    }
}
