impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map : HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let vector = map.entry(*v).or_insert(vec![]);
            vector.push(i as i32);
        }
        
        for v in map.values() {
            if v.len() <= 1 {
                continue;
            }
            
            for i in 0..v.len()-1 {
                if v[i+1] - v[i] <= k {
                    return true;
                }
            }
        }
        
        false
    }
}

pub struct Solution;

fn main() {
    assert_eq!(false, Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
    assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,0,1,1], 1));
    assert_eq!(true, Solution::contains_nearby_duplicate(vec![1,2,3,1], 3));
}