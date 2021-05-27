impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strings: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
        strings.sort_by(|x, y| {
            format!("{}{}", y, x).parse::<i64>().unwrap().cmp(
                &format!("{}{}", x, y).parse::<i64>().unwrap())
        });
        let ret = strings.join(&"".to_string());
        if ret.chars().nth(0).unwrap() == '0' {
            return "0".to_string();
        }
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::largest_number(vec![3i32,30,34,5,9]));
    println!("{}", Solution::largest_number(vec![128i32,12]));
    println!("{}", Solution::largest_number(vec![121i32,12]));
    println!("{}", Solution::largest_number(vec![0i32,0]));
}