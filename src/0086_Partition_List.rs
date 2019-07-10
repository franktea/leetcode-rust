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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut p = &head;
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        while !p.is_none() {
            let val = p.as_ref().unwrap().val;
            p = &p.as_ref().unwrap().next;
            
            if val < x {
                v1.push(val);
            } else {
                v2.push(val);
            }
        }
        
        v1.extend(v2.iter());
        
        vec_to_list(&v1)
    }
}

fn vec_to_list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v.iter().rev() {
        let mut n = ListNode::new(*i);
        n.next = head;
        head = Some(Box::new(n));
    }
    head
}

pub struct Solution;

fn main() {
    Solution::partition(vec_to_list(&vec![1, 2, 3, 4]), 5);
}
