impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strings: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
        use std::cmp::Ordering;
        strings.sort_by(|x, y| {
            for (i, c) in x.chars().enumerate() {
                if i < y.len() && c !=  y.chars().nth(i).unwrap() {
                    return y.chars().nth(i).unwrap().cmp(&c);
                }
            }
            
            //println!("x, y, {}, {}", &x, &y);
            if x.len() == y.len() {
                return Ordering::Equal;
            } else if x.len() < y.len() {
                return y.chars().nth(x.len()).unwrap().cmp(&x.chars().nth(0).unwrap());
            } else {
                return y.chars().nth(0).unwrap().cmp(&x.chars().nth(y.len()).unwrap());
            }
        });
        strings.join(&"".to_string())
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::largest_number(vec![3i32,30,34,5,9]));
    println!("{}", Solution::largest_number(vec![128i32,12]));
}