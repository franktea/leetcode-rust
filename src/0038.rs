impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = "1".to_string();
        for _ in 0..n-1 {
            let mut ret = "".to_string();
            let mut count = 0;
            let mut it = s.chars().peekable(); // use peekable to check next char
            while let Some(c) = it.next() {
                match it.peek() {
                    Some(next) if next == &c => count += 1,
                    _ => {
                        ret.push_str(&(count+1).to_string());
                        ret.push(c);
                        count = 0;
                    }
                }
            }
            s = ret;
        }
        s
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