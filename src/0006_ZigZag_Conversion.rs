impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = num_rows;
        let it = (0..n).collect::<Vec<_>>().into_iter()
            .chain((1..=n-2).rev().collect::<Vec<_>>().into_iter());
        
        let mut vr: Vec<Vec<char>> = vec![vec![]; n as usize];
        for (c, i) in s.chars().zip(it.cycle()) {
            vr[i as usize].push(c);    
        }
        
        vr.into_iter().flatten().collect()
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
}