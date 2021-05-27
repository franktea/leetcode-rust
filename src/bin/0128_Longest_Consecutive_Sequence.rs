impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut ret = 0;
        use std::collections::HashSet;
        let mut set: HashSet<i32> = nums.iter().map(|x| *x).collect();
        while !set.is_empty() {
            let mut v = 1;
            let n = set.iter().nth(0).unwrap().clone(); // get the first element and remove it
            set.remove(&n);
            let mut less = n - 1;
            while !set.is_empty() && set.contains(&less) {
                set.remove(&less);
                less -= 1;
                v += 1;
            }
            let mut more = n + 1;
            while !set.is_empty() && set.contains(&more) {
                set.remove(&more);
                more += 1;
                v += 1;
            }
            if v > ret {
                ret = v;
            }
        }
        
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
}