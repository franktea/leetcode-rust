
struct Chars {
    dest: Vec<i32>,
    cur: Vec<i32>,
}

impl Chars {
    fn new(t: &Vec<u8>) -> Self {
        let mut o = Chars {
            dest: vec![0i32; 128],
            cur: vec![0i32; 128],
        };
        for c in t {
            o.dest[*c as usize] += 1;
        }
        o
    }
    
    fn add_char(&mut self, c: u8) {
        let c = c as usize;
        if self.dest[c] > 0 {
            self.cur[c] += 1;
        }
    }
    
    fn del_char(&mut self, c: u8) {
        let c = c as usize;
        if self.dest[c] > 0 {
            self.cur[c] -= 1;
            assert!(self.cur[c] >= 0);
        }
    }
    
    fn check(&self) -> bool {
        for (x, y) in self.dest.iter().zip(self.cur.iter()) {
            if y < x {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }
        
        let s = s.into_bytes(); 
        let t = t.into_bytes();
        let mut o = Chars::new(&t);
        for i in 0..t.len() {
            o.add_char(s[i]);
        }
        let mut left = 0usize;
        let mut right = t.len()-1;
        let mut distance = std::usize::MAX;
        let mut ret = (0usize, 0usize);
        while right < s.len() && left + t.len() - 1 <= right {
            if o.check() {
                if right - left < distance {
                    distance = right - left;
                    ret = (left, right);
                }
                o.del_char(s[left]);
                left += 1;
            } else {
                right += 1;
                if right < s.len() {
                    o.add_char(s[right]);
                }
            }
        }
        
        if distance < std::usize::MAX { std::str::from_utf8(&s[ret.0..=ret.1]).unwrap().to_string() } else { "".to_string() }
    }
}

// end submission------------------------

struct Solution;

fn main() {
    println!("{}", Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()));
}