impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s == t {
            return true;
        }
        
        if s.len() != t.len() {
            return false;
        }
        
        use std::collections::HashMap;
        let mut map_st = HashMap::new(); // char from s to t
        let mut map_ts = HashMap::new(); // char from t to s
        for (i, cs) in s.chars().enumerate() {
            let ct = t.chars().nth(i).unwrap();
            let bs = map_st.contains_key(&cs);
            let bt = map_ts.contains_key(&ct);
            if bs ^ bt {
                return false;
            }
            
            if bs {
                if &cs != map_ts.get(&ct).unwrap() || &ct != map_st.get(&cs).unwrap() {
                    return false;
                }
            } else {
                map_st.insert(cs, ct);
                map_ts.insert(ct, cs);
            }
        }
        
        true
    }
}

pub struct Solution;

fn main() {
    assert_eq!(true, Solution::is_isomorphic("paper".to_string(), "title".to_string()));
    assert_eq!(false, Solution::is_isomorphic("ab".to_string(), "aa".to_string()));
    assert_eq!(false, Solution::is_isomorphic("aba".to_string(), "baa".to_string()));
}
