impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut bits = vec![0i32; nums.len()+1];
        for i in nums.iter() {
            bits[*i as usize] = 1;
        }
        
        for (i, n) in bits.iter().enumerate() {
            if *n == 0 {
                return i as i32;
            }
        }
        -1
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::missing_number(vec![3,0,1]));
}