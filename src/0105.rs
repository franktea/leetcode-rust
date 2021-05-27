use std::rc::Rc;
use std::cell::RefCell;

fn build(pre: &[i32], inord: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if pre.is_empty() {
        return None;
    }
    
    let root = Rc::new(RefCell::new(TreeNode::new(pre[0])));
    if let Some(index) = inord.iter().position(|&x| x==pre[0]) {
        root.borrow_mut().left = build(&pre[1..index+1], &inord[..index]);
        root.borrow_mut().right = build(&pre[index+1..], &inord[index+1..]);
    }
    
    Some(root)
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build(&preorder, &inorder)
    }
}

// end sumbmission-------------------

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
    println!("{:?}", Solution::build_tree(vec![3,9,20,15,7], vec![9,3,15,20,7]));
}