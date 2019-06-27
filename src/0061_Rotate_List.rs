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
        
        let mut tail = &head;
        for _ in 0..steps {
            tail = &(tail.as_ref().unwrap().next);
        }
        let mut first = &head;
        while tail.as_ref().unwrap().next.is_some() {
            tail = &(tail.as_ref().unwrap().next);
            first = &(first.as_ref().unwrap().next);
        }
        
        // 实在编不过，暂时先把数据存到vector中再生成新的vector
        let mut v = Vec::new();
        {
            let mut cur = &(first.as_ref().unwrap().next);
            while cur.is_some() {
                v.push(cur.as_ref().unwrap().val);
                cur = &(cur.as_ref().unwrap().next);
            }
            
            let mut cur = &head;
            while cur.as_ref().unwrap() != first.as_ref().unwrap().next.as_ref().unwrap() {
                v.push(cur.as_ref().unwrap().val);
                cur = &(cur.as_ref().unwrap().next);
            }
        }
        
        vec_to_list(&v)
    }
}

pub struct Solution;

fn main() {
    let l = vec_to_list(&vec![1, 2]);
    let l2 = Solution::rotate_right(l, 1);
    println!("{:?}", l2);
}