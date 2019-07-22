impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        
        let s = s.as_bytes();
        
        // 查找以第i个字符为起始的最长不重复的字符串，返回值：(不重复字符串长度，下一次查询的起始位置)
        fn get_len(i: usize, s: &[u8]) -> (i32, usize) {
            let mut len = 0;
            let mut bits = [0usize; 128]; // 用数组记录每个字符是否出现过
            let mut to = s.len() - 1;
            for j in i..s.len() {
                let index = s[j] as usize;
                if bits[index] == 0 {
                    bits[index] = j + 1;
                    len += 1;
                } else {
                    to = bits[index]; // 下一次开始搜索的位置，从与当前重复的字符的下一个字符开始
                    break;
                }
            }
            (len, to)
        }
        
        let mut ret = 1;
        let mut i = 0;
        while i < s.len() - 1 {
            //println!("i={}", i);
            let (len, next) = get_len(i, &s);
            if len > ret {
                ret = len;
            }
            i = next;
        }
        
        ret
    }
}

// end submition

pub struct Solution;

fn main() {
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("c".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
}