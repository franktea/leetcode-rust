impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut v = a.iter().map(|x|x*x).collect::<Vec<_>>();
        v.sort();
        v
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), [4,9,9,49,121]);
}