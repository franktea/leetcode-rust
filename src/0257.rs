use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, s: String, mut v: &mut Vec<String>) {
            match root {
                Some(n) => {
                    let left = n.borrow_mut().left.take();
                    let right = n.borrow_mut().right.take();
                    match (left, right) {
                        (Some(l), Some(r)) => {
                            let vl = l.borrow().val;
                            let vr = r.borrow().val;
                            dfs(Some(l), s.clone() + &format!("->{}", vl), &mut v);
                            dfs(Some(r), s + &format!("->{}", vr), &mut v);
                        }
                        (Some(l), None) => {
                            let vl = l.borrow().val;
                            dfs(Some(l), s + &format!("->{}", vl), &mut v);
                        }
                        (None, Some(r)) => {
                            let vr = r.borrow().val;
                            dfs(Some(r), s + &format!("->{}", vr), &mut v);
                        }
                        (None, None) => v.push(s),
                    }
                }
                None => {}, 
            }
        }
        if let Some(node) = root {
            let s = format!("{}", node.borrow().val);
            dfs(Some(node), s, &mut v);
        }
        v
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
    println!("{:?}", Solution::binary_tree_paths(None));
}