use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let chars: HashMap<char, usize> = pattern.chars().enumerate().map(|(i,c)|(c,i)).collect();
        //println!("{:?}", &chars);
        let words_vec = str.split(' ').collect::<Vec<&str>>();
        let words: HashMap<&str,usize> = words_vec.iter().enumerate().map(|(i,w)|(*w,i)).collect();
        //println!("{:?}", &words);
        
        if pattern.len() != words_vec.len() {
            return false;
        }
        
        for (i, c) in pattern.chars().enumerate() {
            if words.get(words_vec[i]).as_ref().unwrap() != &chars.get(&c).unwrap() {
                return false;
            }
        }
        
        true
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()), true);
    assert_eq!(Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()), false);
}
