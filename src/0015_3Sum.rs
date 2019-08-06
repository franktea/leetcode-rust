impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 3 {
            return ret;
        }
        nums.sort();
        use std::collections::HashSet;
        let mut existed: HashSet<(i32, i32, i32)> = HashSet::new(); // 将所有结果保存在这里，去重
        for i in 0..nums.len() - 3 {
            if nums[i] > 0 {
                break;
            }
            for j in i+1..(nums.len() - 2) {
                let sum = 0 - nums[i] - nums[j];
                if sum < nums[j] {
                    break;
                }
                
                if let Some(_) = &nums[j+1..].binary_search(&sum).ok() {
                    break;
                }
                
                let mut v = vec![nums[i], nums[j], sum];
                v.sort();
                if !existed.contains(&(v[0], v[1], v[2])) {
                    existed.insert((v[0], v[1], v[2]));
                    ret.push(v);
                }
            }
        }
        ret
    }
}

// end submission-----------------------

struct Solution;

fn main() {
    println!("{:?}", Solution::three_sum(vec![0]));
}