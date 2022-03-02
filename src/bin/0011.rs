impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{min, max};
        let mut ret = 0;
        for i in 0..height.len() {
            for j in (i+1..height.len()).rev() {
                if (j-i) as i32 * height[i] < ret {
                    break;
                }
                let area = (j-i) as i32 * min(height[i], height[j]);
                ret = max(area, ret);
            }
        }

        ret
    }
}

// end submision-------------

pub struct Solution;

fn main() {
    println!("{}", Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
    println!("{}", Solution::max_area(vec![1,2,1]));
    println!("{}", Solution::max_area(vec![1,2,4,3]));
    println!("{}", Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
}