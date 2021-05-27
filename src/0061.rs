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

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 0 {
            return head;
        }
        
        let mut len = 0i32;
        {
            let mut cur = &head;
            while cur.is_some() {
                len += 1;
                cur = &(cur.as_ref().unwrap().next);
            }
        }
        
        if len <= 1 {
            return head;
        }
        
        let steps = k % len;
        
        if steps == 0 {
            return head;
        }
        
        let mut cur2 = head;  // 移动语义
        let mut cur = &mut cur2;
        // 找到倒数第n个节点
        for _ in 0..(len-steps-1) {
            cur = &mut(cur.as_mut().unwrap().next);
        }
        
        let mut new_head = cur.as_mut().unwrap().next.take(); // 断开
        let mut cur = &mut new_head;
        // 找到尾节点
        while cur.as_ref().unwrap().next.is_some() {
            cur = &mut(cur.as_mut().unwrap().next);
        }
        cur.as_mut().unwrap().next = cur2; // 尾节点指向原来的头结点
        
        new_head
    }
}

pub struct Solution;

fn main() {
    let l = vec_to_list(&vec![1, 2, 3, 4, 5]);
    let l2 = Solution::rotate_right(l, 1);
    println!("{:?}", l2);
}