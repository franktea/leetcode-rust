impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let arr = vec![(1, "I"), (4, "IV"), (5, "V"), (9, "IX"), (10, "X"), 
            (40, "XL"), (50, "L"), (90, "XC"), (100, "C"), 
            (400, "CD"), (500, "D"), (900, "CM"), (1000, "M")];
        
        fn find(n: i32, arr: &Vec<(i32, &'static str)>) -> (i32, &'static str) {
            for (value, s) in arr.iter().rev() {
                if n >= *value {
                    return (*value, *s);
                }
            }
            unreachable!()
        }
        
        // 上次用除法，这次用减法
        let mut ret = "".to_string();
        let mut num = num;
        while num > 0 {
            let (v, s) = find(num, &arr);
            ret.push_str(s);
            num -= v;
        }
        
        ret
    }
}

// end submission------------

struct Solution;

fn main() {
    assert_eq!("MCMXCIV".to_string(), Solution::int_to_roman(1994));
    assert_eq!("LVIII".to_string(), Solution::int_to_roman(58));
}