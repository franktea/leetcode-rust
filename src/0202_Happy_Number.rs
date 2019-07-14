use std::collections::HashSet;

pub fn numbers(n: i32) -> Vec<i32> {
    let mut n = n;
    let mut v: Vec<i32> = vec![];
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    v
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(n);
        loop {
            let l = numbers(n);
            n = l.iter().map(|x| x*x).sum();
            if n == 1 {
                return true;
            }
            if set.contains(&n) {
                return false;
            }
            set.insert(n);
        }      
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::is_happy(19), true);
}