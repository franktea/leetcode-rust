// 计算以第i个字符为中点的字符串的最长回文，从中间往两边检查，复杂度为O(N)。
// 返回值：(最长回文的起点、最长回文的终点)
fn longest_of(i: usize, s: &[u8]) -> (usize, usize) {
    // 先检查奇数长度的字符串，中点只有一个，就是第i个字符
    let mut ret1 = (i, i);
    {
        for (f, t) in (0..=i).rev().zip(i..s.len()) {
            if s[f] != s[t] {
                break;
            }
            ret1 = (f, t);
        }
    }
    
    // 再检查偶数长度的字符串，中点有两个，此处指定为第i、i+1个
    let mut ret2 = (i, i);
    {
        for (f, t) in (0..=i).rev().zip(i+1..s.len()) {
            if s[f] != s[t] {
                break;
            }
            ret2 = (f, t);
        }
    }
    
    return if ret2.1 - ret2.0 > ret1.1 - ret1.0 { ret2 } else { ret1 };
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        
        let mut range = (0, 0);
        let s = s.as_bytes();
        // 遍历每个字符，找出以当前字符为中点的最长回文字符串
        for i in 0..s.len() {
            let r = longest_of(i, &s);
            if r.1 - r.0 > range.1 - range.0 {
                range = r;
            }
        }
        return std::str::from_utf8(&s[range.0..=range.1]).unwrap().to_string();
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
}
