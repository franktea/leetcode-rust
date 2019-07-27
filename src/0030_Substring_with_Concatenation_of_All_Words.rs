// 每个单词都是一样的长度，这才是重点！
use std::collections::HashMap;

// 检查第i个字符开始的字符串是否与指定的一堆单词匹配。
// 将所有单词放在一个hashmap中，匹配到一个就删除一个，缺点是每次调用此函数都要将整个hashmap复制一遍。
fn check(word_len: usize, word_count: usize, s: &str, words: HashMap<&str, i32>) -> bool {
    let mut words = words;
    //println!("{:?}", &words);
    for i in 0..word_count {
        let word = &s[i*word_len..(i+1)*word_len];
        //println!("{}, word: {}", i, &word);
        match words.get_mut(&word) {
            Some(v) => {
                if *v <= 0 {
                    return false;
                }
                *v -= 1;
            }
            _ => return false,
        }
    }
    true
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }
        
        let word_len = words[0].len();
        let word_count = words.len();
        
        if s.len() < word_len * word_count {
            return vec![];
        }
        
        let mut map: HashMap<&str, i32> = HashMap::new();
        for w in words.iter() {
            let k = &w[0..w.len()];
            map.entry(k).and_modify(|x| *x+=1).or_insert(1);
        }
        
        let mut ret = vec![];
        for i in 0..(s.len()-word_len*words.len()+1) {
            if check(word_len, word_count, &s[i..i+word_len*word_count], map.clone()) {
                ret.push(i as i32);
            }
        }
        ret
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_substring("barfoothefoobarman".to_string(), 
        vec!["foo".to_string(),"bar".to_string()]), [0, 9]);
    assert_eq!(Solution::find_substring("wordgoodgoodgoodbestword".to_string(), 
        vec!["word".to_string(),"good".to_string(),"best".to_string(),"word".to_string()]), []);
    assert_eq!(Solution::find_substring("a".to_string(), 
        vec!["a".to_string(),"a".to_string()]), []);
    assert_eq!(Solution::find_substring("wordgoodgoodgoodbestword".to_string(), 
        vec!["word".to_string(),"good".to_string(),"best".to_string(),"good".to_string()]), [8]);
}