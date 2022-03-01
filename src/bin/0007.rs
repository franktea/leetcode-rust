impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let neg = if x < 0 { true } else { false };
        let mut x = x.abs();

        let mut ret = 0i32;
        while x > 0 {
            match ret.checked_mul(10).
                and_then(|val| val.checked_add(x % 10)) {
                Some(r) => ret = r,
                None => return 0,
            }
            x /= 10;
        }

        return if neg { -ret } else { ret };
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-120));
    println!("{}", Solution::reverse(1534236469));
}