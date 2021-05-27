fn mul(index: usize, v: i32, s: &String) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    for _ in 0..index {
        ret.push(0);
    }
    
    let mut left = 0;
    for c in s.chars().rev() {
        let vc = c.to_digit(10).unwrap() as i32;
        let p = v * vc + left;
        left = p / 10;
        ret.push(p % 10);
    }
    
    if left > 0 {
        ret.push(left);
    }
    
    ret
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut vv:Vec<Vec<i32>> = Vec::new(); // 将一堆vec<int>相加
        for (index, c) in num2.chars().rev().enumerate() {
            vv.push(mul(index, c.to_digit(10).unwrap() as i32, &num1));
        }
        
        // 对于所有的vector都将长度补齐到和最长的那个一样长
        let max_len = vv.iter().max_by(|x,y| x.len().cmp(&y.len())).unwrap().len();
        for v in &mut vv {
            for _ in v.len()..max_len {
                v.push(0);
            }
        }
        
        let mut left = 0;
        let mut vsum:Vec<i32> = Vec::new();
        for i in 0..max_len {
            let mut sum = 0;
            for v in &vv {
                sum += v[i];
            }
            sum += left;
            left = sum / 10;
            vsum.push(sum % 10);
        }
        if left > 0 {
            vsum.push(left);
        }
        
        let mut ret = "".to_string();
        for i in vsum.iter() {
            ret.push(std::char::from_digit(*i as u32, 10).unwrap());
        }
        ret = ret.chars().rev().skip_while(|x| x == &'0').collect::<String>(); // 去掉前导的0
        if ret.is_empty() {
            ret = "0".to_string();
        }
        ret
    }
}

// end submission --------------------------

struct Solution;

fn main() {
    assert_eq!("56088".to_string(), Solution::multiply("123".to_string(), "456".to_string()));
    assert_eq!("0".to_string(), Solution::multiply("0".to_string(), "0".to_string()));
    assert_eq!("2040".to_string(), Solution::multiply("408".to_string(), "5".to_string()));
    assert_eq!("0".to_string(), Solution::multiply("9133".to_string(), "0".to_string()));
    assert_eq!("100940".to_string(), Solution::multiply("140".to_string(), "721".to_string()));
}