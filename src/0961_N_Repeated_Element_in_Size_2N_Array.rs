impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for i in a {
            if set.contains(&i) {
                return i;
            }
            set.insert(i);
        }
        return -1;
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::repeated_n_times(vec![2,1,2,5,3,2]), 2);
}