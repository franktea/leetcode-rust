// 参考了这里：
// https://leetcode.com/problems/merge-two-sorted-lists/discuss/212315/Rust-solution-%22why-it's-so-damn-hard-to-implement%22
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) 
        -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(v1), None) => Some(v1),
            (None, Some(v2)) => Some(v2),
            (Some(mut v1), Some(mut v2)) => {
                if v1.val < v2.val {
                    let n = v1.next.take();
                    v1.next = Solution::merge_two_lists(n, Some(v2));
                    Some(v1)
                } else {
                    let n = v2.next.take();
                    v2.next = Solution::merge_two_lists(Some(v1), n);
                    Some(v2)
                }
            }
            _ => None,
        }
    }
}

// end submission

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn vec_to_list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v.iter().rev() {
        let mut node = ListNode::new(*i);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn display(l: Option<Box<ListNode>>) {
    let mut head = &l;
    while head.is_some() {
        print!("{}, ", head.as_ref().unwrap().val);
        head = &(head.as_ref().unwrap().next);
    }
    println!("");
}

fn main() {
    let l = Solution::merge_two_lists(vec_to_list(&vec![1, 3, 5, 7, 9]), 
        vec_to_list(&vec![2, 4, 6, 8, 10]));
    display(l);
    
    let l = Solution::merge_two_lists(vec_to_list(&vec![1, 2, 4]), 
        vec_to_list(&vec![1, 3, 4, 5, 6]));
    display(l);
}