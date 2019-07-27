impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = "1".to_string();
        let mut v:Vec<String> = vec![s.clone()];
        for _ in 0..31 {
            let mut ret = "".to_string();
            let mut cur = None;
            let mut count = 0;
            for c in s.chars() {
                match cur {
                    Some(pre) => {
                        if c == pre {
                            count += 1;
                        } else {
                            ret.push_str(&count.to_string());
                            ret.push(pre);
                            cur = Some(c);
                            count = 1;
                        }
                    }
                    None => {
                        cur = Some(c);
                        count = 1;
                    }
                }
            }
            if let Some(pre) = cur {
                ret.push_str(&count.to_string());
                ret.push(pre);
            }
            v.push(ret.clone());
            s = ret;
        }
        v[n as usize - 1].clone()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_and_say(1));
    println!("{}", Solution::count_and_say(2));
    println!("{}", Solution::count_and_say(3));
    println!("{}", Solution::count_and_say(4));
    println!("{}", Solution::count_and_say(5));
    println!("{}", Solution::count_and_say(6));
    println!("{}", Solution::count_and_say(7));
}