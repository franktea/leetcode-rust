#[derive(PartialEq)]
pub enum State {
    Init,
    LeadingSpaces,
    PlusMinus,
    Num{value: String},
    Dot,
    NumDot,
    NumDotNum,
    NumDotNumE,
    NumDotNumEPlusMinus,
    NumDotNumENum,
    PostSpaces,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::Init;
        for c in s.chars() {
            match state {
                State::Init | State::LeadingSpaces => {
                    match c {
                        '0'..='9' => state = State::Num { value : c.to_string()},
                        ' ' => state = State::LeadingSpaces,
                        '.' => state = State::Dot,
                        '+' | '-' => state = State::PlusMinus,
                        _ => return false,
                    }
                }
                State::PlusMinus => {
                    match c {
                        '0'..='9' => state = State::Num { value : c.to_string()},
                        '.' => state = State::Dot,
                        _ => return false,
                    }
                }
                State::Num {value} => {
                    match c {
                        '0'..='9' => state = State::Num { value : value + &c.to_string()},
                        '.' => state = State::NumDot,
                        'e' => state = State::NumDotNumE,
                        ' ' => state = State::PostSpaces,
                        _ => return false,                        
                    }
                }
                State::Dot => {
                    match c {
                        '0'..='9' => state = State::NumDot,
                        _ => return false,                         
                    }
                }
                State::NumDot | State::NumDotNum => {
                    match c {
                       '0'..='9' => state = State::NumDotNum,
                       'e' => state = State::NumDotNumE,
                       ' ' => state = State::PostSpaces,
                        _ => return false, 
                    }
                }
                State::NumDotNumE => {
                    match c {
                       '0'..='9' => state = State::NumDotNumENum,
                       '+' | '-' => state = State::NumDotNumEPlusMinus,
                        _ => return false,                         
                    }
                }
                State::NumDotNumEPlusMinus => {
                    match c {
                       '0'..='9' => state = State::NumDotNumENum,
                        _ => return false,                         
                    }                    
                }
                State::NumDotNumENum => {
                    match c {
                        '0'..='9' => state = State::NumDotNumENum,
                        ' ' => state = State::PostSpaces,
                        _ => return false,
                    }
                }
                State::PostSpaces => {
                    match c {
                        ' ' => {}
                        _ => return false,
                    }
                }
            }
        }
        
        match state {
            State::Init | State::LeadingSpaces | State::PlusMinus | State::Dot | State::NumDotNumE | State::NumDotNumEPlusMinus => return false,
            _ => return true,
        }
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::is_number(" 0.1 ".to_string()), true);
    assert_eq!(Solution::is_number("0".to_string()), true);
    assert_eq!(Solution::is_number("abc".to_string()), false);
    assert_eq!(Solution::is_number("1 a".to_string()), false);
    assert_eq!(Solution::is_number("2e10".to_string()), true);
    assert_eq!(Solution::is_number(" -90e3   ".to_string()), true);
    assert_eq!(Solution::is_number(" 1e".to_string()), false);
    assert_eq!(Solution::is_number("e3".to_string()), false);
    assert_eq!(Solution::is_number(" 6e-1".to_string()), true);
    assert_eq!(Solution::is_number(" 99e2.5 ".to_string()), false);
    assert_eq!(Solution::is_number("53.5e93".to_string()), true);
    assert_eq!(Solution::is_number(" --6 ".to_string()), false);
    assert_eq!(Solution::is_number("-+3".to_string()), false);
    assert_eq!(Solution::is_number("95a54e53".to_string()), false);
    assert_eq!(Solution::is_number("1 ".to_string()), true);
    assert_eq!(Solution::is_number("4e+".to_string()), false);
    assert_eq!(Solution::is_number("+ 1".to_string()), false);
}