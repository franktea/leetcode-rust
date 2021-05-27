impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        match num % 9 {
            0 => if num > 0 { 9 } else { 0 },
            i => i,
        }    
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::add_digits(0), 0);
    assert_eq!(Solution::add_digits(18), 9);
    assert_eq!(Solution::add_digits(15), 6);
}