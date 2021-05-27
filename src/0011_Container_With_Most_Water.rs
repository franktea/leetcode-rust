impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        use std::cmp::{max, min};
        for i in 0..height.len() {
            for j in i+1..height.len() {
                let area = (j-i) as i32 * min(height[i], height[j]);
                ret = max(ret, area);
            }
        }
        ret
    }
}

// end submision-------------

pub struct Solution;

fn main() {
    println!("{}", Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
}