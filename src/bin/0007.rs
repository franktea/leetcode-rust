impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let negtive = x != x.abs();
        let mut x = x.abs();
        let mut ret = 0;
        while x > 0 {
            if (std::i32::MAX - x % 10) / 10 < ret {
                return 0;
            }
            ret = ret * 10 + x % 10;
            x /= 10;
        }
        
        return if negtive { -ret } else { ret };
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-120));
    println!("{}", Solution::reverse(1534236469));
}