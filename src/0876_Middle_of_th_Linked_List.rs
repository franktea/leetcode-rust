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

fn to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in arr {
        let mut new_node = ListNode::new(*i);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    head
}

fn main() {
    let node = to_list(&[0, 1, 2, 3, 4, 5, 6]);
    let mid = Solution::middle_node(node);
    println!("mid val is: {}", mid.as_ref().unwrap().val);
}
