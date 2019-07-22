use std::collections::VecDeque;

fn list_to_stack(mut list: Option<Box<ListNode>>) -> VecDeque<Box<ListNode>> {
    let mut stack = VecDeque::new();
    while let Some(mut node) = list.take() {
        list = node.next.take();
        stack.push_front(node);
    }
    stack
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack1 = list_to_stack(l1);
        let mut stack2 = list_to_stack(l2);
        let mut head = None;
        fn add_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            let mut n = ListNode::new(val);
            n.next = head;
            Some(Box::new(n))
        }
        let mut left = 0;
        loop {
            let node1 = stack1.pop_front();
            let node2 = stack2.pop_front();
            if node1.is_none() && node2.is_none() {
                if left > 0 {
                    head = add_node(head, left);
                }
                break;
            }
            
            let mut sum = left;
            if let Some(n) = node1 {
                sum += n.val;
            }
            if let Some(n) = node2 {
                sum += n.val;
            }
            left = sum / 10;
            head = add_node(head, sum % 10);
        }
        head
    }
}

//end submition--------------

// Definition for singly-linked list.
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

pub struct Solution;

fn main() {
    
}

