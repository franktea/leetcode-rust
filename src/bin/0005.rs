// 以index为中心的最长回文
pub fn longest_palindrome_of(index: usize, sv: &Vec<char>) -> std::ops::Range<usize> {
    // 基数长度的回文
    let mut r1 = index..(index+1);
    for (i, j) in (0..=index).rev().zip(index..sv.len()) {
        if sv[i] != sv[j] {
            break;
        }
        r1 = i..(j+1);
    }

    // 偶数长度的回文
    for (i, j) in (0..=index).rev().zip(index+1..sv.len()) {
        if sv[i] != sv[j] {
            break;
        }

        if &r1.len() < &(i..j+1).len() {
            r1 = i..(j+1);
        }
    }

    r1
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut r = 0usize..0usize;
        for i in 0..s.len() {
            let r2 = longest_palindrome_of(i, &s);
            if r.len() < r2.len() {
                r = r2;
            }
        }

        s[r].into_iter().collect::<String>()
    }
}

// end submission---------------

pub struct Solution;

fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
    println!("{}", Solution::longest_palindrome("cbbd".to_string()));
    println!("{}", Solution::longest_palindrome("a".to_string()));
}
