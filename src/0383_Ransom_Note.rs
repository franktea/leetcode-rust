impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            map.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }
        
        for c in ransom_note.chars() {
            if !map.contains_key(&c) {
                return false;
            }
            map.entry(c).and_modify(|x| *x-=1);
            if map.get(&c).unwrap() < &0 {
                return false;
            }
        }
        true
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
    assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
    assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
}