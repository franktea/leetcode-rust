impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        // 一层一层地计算，nums中的每个值为一层，第n层共有2^n个值，但是考虑到可能会重复，
        // 所以用hashmap来保存每一层的值，key为值，value为该值重复的次数。
        use std::collections::HashMap;
        // 计算第index层所有可能的取值情况，返回对应的hashmap
        fn calc(nums: &Vec<i32>, index: usize, old_map: HashMap<i32, i32>) -> HashMap<i32, i32> {
            let mut new_map: HashMap<i32, i32> = HashMap::new();
            for (k, v) in &old_map {
                let a = k + nums[index];
                new_map.entry(a).and_modify(|x| *x+=*v).or_insert(*v);
                let b = k - nums[index];
                new_map.entry(b).and_modify(|x| *x+=*v).or_insert(*v);
            }
            //println!("index={}, map={:?}", index, &new_map);
            new_map
        }
        
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        for i in 0..nums.len() {
            map = calc(&nums, i, map);
        }
        
        if let Some(v) = map.get(&s) {
            return *v;
        }
        
        0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_target_sum_ways(vec![1i32, 1, 1, 1, 1], 3));
}
