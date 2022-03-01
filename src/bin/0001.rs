use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match map.get(v) {
                Some(&index) => { return vec![index, i as i32]; }
                _ => { map.insert(target-v, i as i32); }
            }
        }
        vec![]
    }
}

// end submission----------

pub struct Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2,7,11,15], 9));
}