impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let roman = vec![('I', 1),
                         ('V', 5),
                         ('X', 10),
                         ('L', 50),
                         ('C', 100),
                         ('D', 500),
                         ('M', 1000)];
        let map: HashMap<_,_> = roman.into_iter().collect();
        let mut ret = 0;
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if let Some(n) = it.peek() {
                let v = map.get(&c).unwrap();
                if v < map.get(n).unwrap() {
                    ret -= v;
                } else {
                    ret += v;
                }
            } else {
                ret += map.get(&c).unwrap();
            }
        }
        ret
    }
}


//end submission-----------------

struct Solution;

fn main() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}