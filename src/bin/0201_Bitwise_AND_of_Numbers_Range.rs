impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if m == n {
            return m;
        }
        let mut ret = m;
        for i in m+1..=n {
            ret &= i;
        }
        ret        
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::range_bitwise_and(0, 1));
}