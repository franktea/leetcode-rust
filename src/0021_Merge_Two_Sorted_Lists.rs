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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) 
        -> Option<Box<ListNode>> {
        let mut cur1 = &l1;
        let mut cur2 = &l2;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;
        while cur1.is_some() && cur2.is_some() {
            let mut node = ListNode{val: 0, next: None};
            if cur1.as_ref().unwrap().val > cur2.as_ref().unwrap().val {
                node.val = cur2.as_ref().unwrap().val;
                cur2 = &(cur2.as_ref().unwrap().next);
            } else {
                node.val = cur1.as_ref().unwrap().val;
                cur1 = &(cur1.as_ref().unwrap().next);
            }
            tail.as_mut().unwrap().next = Some(Box::new(node));
            tail = &mut(tail.as_mut().unwrap().next);
        }

        // 将链表节点ln后面的所有节点复制到tail的后面
        let f = |mut ln: &Option<Box<ListNode>>, mut tail: &mut Option<Box<ListNode>>| {
	        while ln.is_some() {
		        let node = ListNode::new(ln.as_ref().unwrap().val);
                tail.as_mut().unwrap().next = Some(Box::new(node));
                tail = &mut(tail.as_mut().unwrap().next);
                ln = &(ln.as_ref().unwrap().next);
            }
        };     
        
        if cur1.is_some() {
            f(cur1, tail);
        } else if cur2.is_some() {
            f(cur2, tail);
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

fn display(l: Option<Box<ListNode>>) {
    let mut head = &l;
    while head.is_some() {
        print!("{}, ", head.as_ref().unwrap().val);
        head = &(head.as_ref().unwrap().next);
    }
}

fn main() {
    let l = Solution::merge_two_lists(vec_to_list(&vec![1, 3, 5, 7, 9]), 
        vec_to_list(&vec![2, 4, 6, 8, 10]));
    display(l);
}