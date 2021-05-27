use std::rc::Rc;
use std::cell::RefCell;

fn build(inor: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if inor.is_empty() {
        return None;
    }
    
    let val = post[post.len()-1];
    let root = Rc::new(RefCell::new(TreeNode::new(val)));
    if let Some(index) = inor.iter().position(|&x| x==val) {
        root.borrow_mut().left = build(&inor[..index], &post[..index]);
        root.borrow_mut().right = build(&inor[index+1..], &post[index..post.len()-1]);
    }
    
    Some(root)
}

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build(&inorder, &postorder)
    }
}

// end submission---------------------------

// Definition for a binary tree node
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
            right: None,
        }
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::build_tree(vec![9,3,15,20,7], vec![9,15,7,20,3]));
}
