impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }
        
        let mut ret = 0;
        let mut left = false;
        for v in map.values() {
            if *v & 1 == 1 {
                left = true;
            }
            if v > &1 {
                ret += v/2 * 2;
            }
        }
        
        if left {
            ret += 1;
        }
        
        ret
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
}