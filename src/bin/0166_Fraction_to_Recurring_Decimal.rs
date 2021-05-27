impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = numerator as i64;
        let denominator = denominator as i64;
        let negtive = numerator * denominator < 0;
        let numerator = numerator.abs();
        let denominator = denominator.abs();
 
        let ret = numerator / denominator; // 整数部分
        let mut left = numerator % denominator;
        if left == 0 {
            let mut s = "".to_string();
            if negtive {
                s.push_str(&"-".to_string());
            }
            s.push_str(&ret.to_string());
            return s;
        }
        
        // 小数部分自己模拟竖式运算
        
        use std::collections::HashMap;
        // 所有的除数存起来，遇到重复的表示开始循环小数
        let mut left_map: HashMap<i64, usize> = HashMap::new(); 
        let mut decimals: Vec<i64> = Vec::new();
        
        fn check(left: i64, ret: i64, negtive: bool,
            left_map: &HashMap<i64, usize>, decimals: &Vec<i64>) -> Option<String> {
            if let Some(index) = left_map.get(&left) {
                //println!("{}, {:?}, {:?}", left, &left_map, &decimals);
                let mut s = "".to_string();
                if negtive {
                    s.push_str(&"-".to_string());
                }
                s.push_str(&ret.to_string());
                s.push_str(&".".to_string());
                for i in 0..decimals.len() {
                    if i == *index {
                        s.push_str(&"(".to_string());
                    }
                    s.push_str(&decimals[i].to_string());
                }
                s.push_str(&")".to_string());
                return Some(s);
            }
            None
        }
        
        loop {
            left *= 10;
            if let Some(s) = check(left, ret, negtive, &left_map, &decimals) {
                return s;
            }
            
            left_map.insert(left, decimals.len());
            decimals.push(left / denominator);
            left %= denominator;
            if left == 0 {
                break;
            }
        }
        
        let mut s = "".to_string();
        if negtive {
            s.push_str(&"-".to_string());
        }
        s.push_str(&ret.to_string());
        s.push_str(&".".to_string());
        for i in 0..decimals.len() {
            s.push_str(&decimals[i].to_string());
        }
        return s;
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::fraction_to_decimal(1, 90));
    println!("{:?}", Solution::fraction_to_decimal(1, 2));
    println!("{:?}", Solution::fraction_to_decimal(2, 3));
    println!("{:?}", Solution::fraction_to_decimal(-2147483648, -1));
    println!("{:?}", Solution::fraction_to_decimal(1, 17));
    println!("{:?}", Solution::fraction_to_decimal(1, 6));
}