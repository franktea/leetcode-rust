impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s: Vec<_> = s.chars().skip_while(|c| c == &' ')
            .enumerate()
            .take_while(|(i, c)| {
                match (i, c) {
                    (0, '+' | '-') => true,
                    (_, c) if c.is_numeric() => true,
                    _ => false,
                }})
            .map(|(_,c)| c)
            .collect();

        let mut neg = 1;
        let mut nums = &s[..];
        match &s[..] {
            [] | ['+' | '-'] => return 0,
            ['+', tail @ ..] => { neg = 1; nums = tail; },
            ['-', tail @ ..] => { neg = -1; nums = tail; },
            [tail @ ..] => { nums = tail; },
        }

        let mut ret = 0i32;
        for c in nums.iter() {
            let n = c.to_digit(10).unwrap() as i32;
            match neg {
                1 => { 
                    if (i32::MAX - n) / 10 < ret {
                        return i32::MAX; 
                    }
                    ret = ret * 10 + n;
                    },
                -1 => {
                    if (i32::MIN + n) / 10 > ret {
                        return i32::MIN;
                    }
                    ret = ret * 10 - n;
                },
                _ => panic!(),
            }
        }

        ret
    }
}

// end submission---------------

pub struct Solution;

fn main() {
    println!("{}", Solution::my_atoi("   -423456".to_string()));
    println!("{}", Solution::my_atoi("4193 with words".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("+91283472332".to_string()));
    println!("{}", Solution::my_atoi("-91283472332".to_string()));
    println!("{}", Solution::my_atoi("+-12".to_string()));
    println!("{}", Solution::my_atoi("   -42".to_string()));
}