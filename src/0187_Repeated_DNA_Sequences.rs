impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        if s.len() < 10 {
            return ret;
        }
        use std::collections::HashMap;
        let mut map: HashMap<&str, i32> = HashMap::new();
        for i in 0..s.len()+1-10 {
            let tmp = &s[i..i+10];
            map.entry(tmp).and_modify(|x| {
                if *x == 0 {
                    ret.push(tmp.to_string());
                    *x = 1;
                }
            }).or_insert(0);
        }
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()));
    assert_eq!(Solution::find_repeated_dna_sequences("AAAAAAAAAAA".to_string()), ["AAAAAAAAAA"]);
    assert_eq!(Solution::find_repeated_dna_sequences("AAAAAAAAAAAA".to_string()), ["AAAAAAAAAA"]);
}