impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let v: Vec<_> = x.to_string().chars().collect();
        return v.iter().rev().take(v.len()/2).eq(v.iter().take(v.len()/2));
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(-121));
    println!("{}", Solution::is_palindrome(10));
}