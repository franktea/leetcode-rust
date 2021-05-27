impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        for (i, c) in s.chars().enumerate() {
            if map.get(&c).unwrap() == &1 {
                return i as i32;
            }
        }
        
        -1
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
}