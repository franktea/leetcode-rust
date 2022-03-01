impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = num_rows as usize;
        let mut rows: Vec<String> = vec!["".to_string(); n];
        let index = (0..n).chain((1..n-1).rev()); // 行的序列
        s.chars().zip(index.cycle()).into_iter().for_each(|(c,i)| rows[i].push(c));
        rows.join("")
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
}