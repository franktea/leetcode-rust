
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        
        let neg = (dividend > 0) ^ (divisor > 0);
        let dividend = dividend as i64;
        let divisor = divisor as i64;
        
        let mut a = dividend.abs();
        let b = divisor.abs();
        
        if a < b {
            return 0;
        }
        
        let mut v = vec![b];
        let mut item = b;
        while item <= std::i32::MAX as i64 {
            match item.checked_add(item) {
                Some(sum) => { 
                    v.push(sum);
                    item = sum;
                }
                _ => break,
            }
        }
        //println!("a={}, b={}, v={:?}", a, b, &v);
        
        fn find_index(v: &Vec<i64>, n: i64) -> usize {
            for i in (0..v.len()).rev() {
                if v[i] <= n {
                    return i;
                }
            }
            0
        }
        
        let mut ret = 0i64;
        while a >= b {
            let i = find_index(&v, a);
            ret += 1i64 << i;
            a -= v[i];  
        }
        
        if (neg && (ret > 2147483648)) || (!neg && (ret > 2147483647)) {
            return 2147483647i32;
        }
        
        return if neg { -ret as i32 } else { ret as i32 }; 
    }
}

// end submission----------------------

struct Solution;

fn main() {
    assert_eq!(-2147483648, Solution::divide(-2147483648, 1));
    assert_eq!(0, Solution::divide(-1010369383, -2147483648));
    assert_eq!(2147483647, Solution::divide(-2147483648, -1));
    assert_eq!(1, Solution::divide(1, 1));
    assert_eq!(3, Solution::divide(10, 3));
    assert_eq!(-2, Solution::divide(7, -3));
}