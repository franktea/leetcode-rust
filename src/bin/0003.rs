use std::{collections::HashSet};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ret = 0usize;
        let mut p1 = 0usize; // 双指针
        let mut p2 = 0usize;
        let v: Vec<char> = s.chars().collect();
        let mut set: HashSet<char> = HashSet::new();
        while p1 < v.len() && p2 < v.len() {
            while set.contains(&v[p2]) && p1 <= p2 && p1 < v.len() {
                set.remove(&v[p1]);
                p1 += 1;
            }

            set.insert(v[p2]);
            p2 += 1;

            if ret < p2 - p1 {
                ret = p2 - p1;
            }
        }
        ret as i32
    }
}

// end submission

pub struct Solution;

fn main() {
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("c".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
}