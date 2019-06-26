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

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &(slow.as_ref().unwrap().next);
            fast = &(fast.as_ref().unwrap().next.as_ref().unwrap().next);
        }
        slow.clone()
    }
}


pub struct Solution;

fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v.iter().rev() {
        let mut node = ListNode::new(*i);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn main() {
    let head = vec_to_list(vec![1i32, 2, 3, 4, 5, 6]);
    let mid = Solution::middle_node(head);
    println!("val = {}", mid.unwrap().val);
}