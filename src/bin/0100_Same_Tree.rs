use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, 
        q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(x), Some(y)) => {
                let mut a = x.borrow_mut();
                let mut b = y.borrow_mut();
                if a.val != b.val {
                    return false;
                }
                return Solution::is_same_tree(a.left.take(), b.left.take()) &&
                    Solution::is_same_tree(a.right.take(), b.right.take());
            }
            _ => false,
        }
    }
}

// end submission-----------
// Definition for a binary tree node.
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

struct Solution;

fn main() {
    
}