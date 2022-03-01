impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut left: i32 = 0;
        let mut ret: Option<Box<ListNode>> = None;
        loop {
            if l1.is_none() && l2.is_none() && left == 0 {
                break;
            }

            if let Some(node) = l1.take() {
                left += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2.take() {
                left += node.val;
                l2 = node.next;
            }

            let mut node = Box::new(ListNode::new(left % 10));
            if ret.is_some() {
                node.next = ret;
            }
            ret = Some(node);
            
            left /= 10;
        }

        // reverse the list
        let mut head = None;
        while let Some(mut node) = ret.take() {
            ret = node.next;
            node.next = head;
            head = Some(node);
        }

        head
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

fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for n in vec.into_iter().rev() {
        let mut node = Box::new(ListNode::new(n));
        node.next = head;
        head = Some(node);
    }
    head
}

fn main() {
    let r1 = Solution::add_two_numbers(
        to_list(vec![2,4,3]), 
        to_list(vec![5,6,4]));
    println!("{:?}", r1);

    let r1 = Solution::add_two_numbers(
        to_list(vec![9,9,9,9,9,9,9]), 
        to_list(vec![9,9,9,9]));
    println!("{:?}", r1);
}

