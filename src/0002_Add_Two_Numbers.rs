impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = None;
        
        fn add_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            let mut n = ListNode::new(val);
            n.next = head;
            Some(Box::new(n))
        }
        
        let mut left = 0;
        
        loop {
            if l1.is_none() && l2.is_none() {
                if left > 0 {
                   head = add_node(head, left);
                }
                break; 
            }
            
            let mut sum = left;
            if let Some(n) = l1 {
                sum += n.val;
                l1 = n.next;
            }
            if let Some(n) = l2 {
                sum += n.val;
                l2 = n.next;
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

