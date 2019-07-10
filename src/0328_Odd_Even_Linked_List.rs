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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if ! head.is_some() {
            return head;
        }
        
        let mut first = head.as_ref();
        let mut second = head.as_ref().unwrap().next
        int i = 
        
        None
    }
}

pub struct Solution;

fn to_list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in v {
        let mut node = ListNode::new(*i);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn main()
{
    let l = vec![1, 2, 3, 4, 5];
    let a = Solution::odd_even_list(to_list(&l));
    println!("{:?}", a);
}