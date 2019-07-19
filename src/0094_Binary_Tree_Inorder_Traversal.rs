use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut root = root;
        let mut ret: Vec<i32> = Vec::new();
        let mut stack = VecDeque::new();
        while root.is_some() || !stack.is_empty() {
            if let Some(n) = root {
                if n.borrow().left.is_some() {
                    let left = n.borrow_mut().left.take();
                    stack.push_front(Some(n));
                    root = left;
                } else {
                    ret.push(n.borrow().val);
                    root = n.borrow_mut().right.take();
                }
            } else {
                root = stack.pop_front().unwrap();
            }
        }
        ret
    }
}

//--------- end submit

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
 
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub struct Solution;

fn main() {
    
}