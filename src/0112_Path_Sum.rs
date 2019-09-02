use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        fn calc(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, dest: i32) -> bool {
            match root {
                Some(n) => {
                    let s = sum + n.borrow().val;
                    let left = n.borrow_mut().left.take();
                    let right = n.borrow_mut().right.take();
                    match (left, right) {
                        (None, None) => s == dest, // leaf
                        (Some(l), Some(r)) => {
                            return calc(Some(l), s, dest) ||
                                calc(Some(r), s, dest);
                        }
                        (Some(l), None) => calc(Some(l), s, dest),
                        (None, Some(r)) => calc(Some(r), s, dest),
                    }
                }
                _ => return false,
            }
        }
        
        return calc(root, 0, sum);
    }
}

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
    println!("{}", Solution::has_path_sum(None, 0));
}