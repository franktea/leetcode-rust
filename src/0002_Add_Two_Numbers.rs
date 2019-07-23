impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = None;
        
        let mut left = 0;
        
        loop {
            if l1.is_none() && l2.is_none() && left == 0{
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
            let mut node = ListNode::new(sum % 10);
            node.next = head;
            head = Some(Box::new(node));
        }
        
        // reverse the list
        let mut tail = None;
        while let Some(mut n) = head.take() {
            head = n.next;
            n.next = tail;
            tail = Some(n);
        }
        tail
    }
}

//end submission--------------

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

