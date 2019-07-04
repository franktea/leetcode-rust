impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let set1: HashSet<_> = nums1.iter().cloned().collect();
        let set2: HashSet<_> = nums2.iter().cloned().collect();
        
        let ret: Vec<_> = set1.intersection(&set2).cloned().collect();
        ret
    }
}

struct Solution;

fn main() {
    let v = Solution::intersection(vec![1, 2, 3, 3, 5], vec![2, 3, 2, 3]);
    println!("{:?}", v);
}