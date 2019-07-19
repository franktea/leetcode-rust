fn check(i: usize, gas: &Vec<i32>, cost: &Vec<i32>) -> bool {
    let mut total = gas[i];
    for x in 1usize..=gas.len() {
        let next = (i + x) % gas.len();
        let pre = (next + gas.len() - 1) % gas.len();
        total -= cost[pre];
        if total < 0 {
            return false;
        }
        total += gas[next];
    }
    true
}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for i in 0..gas.len() {
            if check(i, &gas, &cost) {
                return i as i32;
            }
        }
        -1
    }
}

pub struct Solution;

pub fn main() {
    assert_eq!(Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
}