impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m1: HashMap<i32, i32> = HashMap::new();
        for i in nums1 {
            m1.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut m2: HashMap<i32, i32> = HashMap::new();
        for i in nums2 {
            m2.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut ret:Vec<i32> = Vec::new();
        use std::cmp::min;
        for k in m1.keys() {
            m2.entry(*k).and_modify(|v| {
                let times = min(*v, *m1.get(k).unwrap());
                for _ in 0..times {
                    ret.push(*k);
                }
            });
        }
        ret
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::intersect(vec![1,2,2,1], vec![2,2]), vec![2,2]);
    assert_eq!(Solution::intersect(vec![4,9,5], vec![9,4,9,8,4]), vec![4,9]);
}
