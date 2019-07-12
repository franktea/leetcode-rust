use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|x| *x+=1).or_insert(1);
        }
        
        let mut v:Vec<(&char, &i32)> = map.iter().collect();
        v.sort_by(|i, j| j.1.cmp(&i.1));
        let mut ret = "".to_string();
        for (c, n) in v {
            for _ in 0..*n {
                ret.push(c.clone());
            }
        }
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::frequency_sort("cccaaa".to_string()));
}