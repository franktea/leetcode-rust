impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }
        
        if n == 1 {
            return vec![0, 1];
        }
        
        // 找规律，二进制每次向前进一位时只要将上一组数据逆序，然后在前面加上1即可。
        let mut ret = vec![0, 1];
        for i in 1..n {
            let v2 = ret.clone();
            for t in v2.iter().rev() {
                ret.push((1 << i) | t);
            }
        }
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::gray_code(2));
    println!("{:?}", Solution::gray_code(0));
    println!("{:?}", Solution::gray_code(7));
}