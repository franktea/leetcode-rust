impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut ret = 0;
        let mut sum1: HashMap<i32, i32> = HashMap::new();
        for i in a.iter() {
            for j in b.iter() {
                let v = i + j;
                sum1.entry(v).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        let mut sum2: HashMap<i32, i32> = HashMap::new();
        for i in c.iter() {
            for j in d.iter() {
                let v = i + j;
                sum2.entry(v).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        
        for (k, v) in sum1.iter() {
            sum2.entry(-k).and_modify(|x| ret += v * *x);
        }
        ret
    }
}

pub struct Solution;

fn main() {
    
}
