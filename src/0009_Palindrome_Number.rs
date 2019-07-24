impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let arr = s.as_bytes();
        for (c1, c2) in (0..arr.len()/2).into_iter().zip(
            (arr.len()/2..arr.len()).rev().into_iter()) {
            if arr[c1] != arr[c2] {
                return false;
            }        
        }
        true
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(-121));
    println!("{}", Solution::is_palindrome(10));
}