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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {val: 0i32, next: head}));
        let mut len = 0;
        {
            let mut cur = &dummy.as_ref().unwrap().next;
            while cur.is_some() {
                len += 1;
                cur = &(cur.as_ref().unwrap().next);
            }
        }
        {
            let steps = len - n;
            let mut cur = dummy.as_mut();
            for _ in 0..steps {
                cur = cur.unwrap().next.as_mut();
            }
            let next = cur.as_mut().unwrap().next.as_mut().unwrap().next.clone();
            cur.as_mut().unwrap().next = next;
        }
        dummy.unwrap().next
    }
}

pub struct Solution;

fn vec_to_list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v.iter().rev() {
        let mut node = ListNode::new(*i);
        node.next = head;
        head = Some(Box::new(node));
    }
    head   
}

fn display(head: Option<Box<ListNode>>) {
    let mut tail = &head;
    while tail.is_some() {
        print!("{}, ", tail.as_ref().unwrap().val);
        tail = &(tail.as_ref().unwrap().next);
    }
}

fn main() {
    let l = vec_to_list(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let l2 = Solution::remove_nth_from_end(l, 5);
    display(l2);
}