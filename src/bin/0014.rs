impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let chars = strs.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let shortest = chars.iter().map(|s| s.len()).min().unwrap_or(0); // 最短的长度
        let mut ret: Vec<char> = Vec::new();
        'outter: for i in 0..shortest {
            for j in 1..chars.len() {
                if chars[j][i] != chars[0][i] {
                    break 'outter;
                }
            }
            ret.push(chars[0][i]);
        }
        
        ret.iter().collect::<String>()
    }
}

// end of submition

pub struct Solution;

fn main() {
    println!("{}", Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
}