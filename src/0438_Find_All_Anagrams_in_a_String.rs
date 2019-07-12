use std::collections::HashMap;

fn check(s: &str, m: HashMap<char, i32>) -> bool {
    let mut m = m;
    use std::collections::hash_map::Entry;
    for c in s.chars() {
        match m.entry(c) {
            Entry::Occupied(mut o) => {
                if o.get() <= &0 {
                    return false;
                }
                let v = o.get_mut();
                *v -= 1;
            }
            Entry::Vacant(_) => {
                return false;
            }
        }
    }
    
    for (_, v) in m.iter() {
        if v != &0 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in p.chars() {
            map.entry(c).and_modify(|x|*x+=1).or_insert(1);
        }
        
        let mut ret = Vec::new();
        if s.len() < p.len() {
            return ret;
        }
        
        for i in 0..(s.len()+1-p.len()) {
            if check(&s[i..i+p.len()], map.clone()) {
                ret.push(i as i32);
            }
        }
        
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()));
}
