impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a_count = 0i32;
        let mut last: char = '0';
        let mut count = 0i32;
        for c in s.chars() {
            match c {
                'P' => {
                    last = '0';
                    count = 0;
                }
                'A' => {
                    a_count += 1;
                    if a_count > 1 {
                        return false;
                    }
                    last = '0';
                    count = 0;
                }
                _ => {
                    if last == 'L' {
                        count += 1;
                        if count > 2 {
                            return false;
                        }
                    } else {
                        last = 'L';
                        count = 1;
                    }
                }
            }
        }
        true   
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::check_record("PPALLP".to_string()), true);
}