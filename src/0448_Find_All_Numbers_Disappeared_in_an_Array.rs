impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut set: HashSet<_> = (1i32..=(nums.len() as i32)).collect();
        for i in nums {
            set.remove(&i);
        } 
        let v: Vec<i32> = set.iter().cloned().collect();
        v
    }
}

pub struct Solution;

fn main() {
    let v = vec![4,3,2,7,8,2,3,1];
    println!("{:?}", Solution::find_disappeared_numbers(v));
}