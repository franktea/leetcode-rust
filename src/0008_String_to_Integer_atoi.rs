enum State {
    Init,
    ExpectNumber, // 已经碰到了+或者-，下一个字符必须是数字
    Number(i32),
}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut state = State::Init;
        let mut neg = 1; // 正负符号，为1或-1，最后的结果乘此数即可
        for c in str.chars() {
            match c {
                ' ' => match state {
                    State::Init => {},
                    State::ExpectNumber =>  return 0,
                    State::Number(n) => return neg * n,
                }
                '+' | '-' => match state {
                    State::Init => {
                        state = State::ExpectNumber;
                        if c == '-' {
                            neg = -1;
                        }
                    }
                    State::ExpectNumber => return 0,
                    State::Number(n) => return neg * n,
                }                   
                '0'..='9' => {
                    let digit = c.to_digit(10).unwrap() as i32;
                    match state {
                        State::Init | State::ExpectNumber => state = State::Number(digit),
                        State::Number(n) => match n.checked_mul(10).and_then(|x|x.checked_add(digit)) {
                            Some(number) => state = State::Number(number),
                            _ => return if neg < 0 {std::i32::MIN} else {std::i32::MAX},
                        }
                    }                    
                }
                _ => match state {
                    State::Init | State::ExpectNumber => return 0,
                    State::Number(n) => return neg * n,
                }                   
            }
        }
        
        match state {
            State::Number(n) => return neg * n,
            _ => return 0,
        }
    }
}

// end submission---------------

pub struct Solution;

fn main() {
    println!("{}", Solution::my_atoi("   -423456".to_string()));
    println!("{}", Solution::my_atoi("4193 with words".to_string()));
    println!("{}", Solution::my_atoi("words and 987".to_string()));
    println!("{}", Solution::my_atoi("-91283472332".to_string()));
}