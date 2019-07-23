impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = num_rows as usize;
        let mut v: Vec<_> = (0..n).collect();
        if n > 2 {
            let v2: Vec<_> = (1..=n-2).rev().collect();
            v.extend(v2);
        }
        
        let mut vr: Vec<Vec<char>> = vec![vec![]; n];
        for (c, i) in s.chars().zip(v.iter().cycle()) {
            vr[*i].push(c);    
        }
        
        vr.into_iter().flatten().collect()
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
}