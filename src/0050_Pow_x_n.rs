impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::my_pow(2.00000f64, 10i32));
    println!("{}", Solution::my_pow(2.10000f64, 3i32));
    println!("{}", Solution::my_pow(2.00000f64, -2i32));
}