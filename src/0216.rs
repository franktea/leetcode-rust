pub fn dfs(depth: i32, k: i32, n: i32, mut v: &mut Vec<i32>, mut ret: &mut Vec<Vec<i32>>) {
    if depth == 9 {
        let mut count = 0;
        let mut sum = 0;
        for i in v.iter() {
            if *i != 0 {
                count += 1;
                sum += *i;
            }
        }
        if sum == n && count == k {
            let items:Vec<i32> = v.iter().map(|x| x.clone()).filter(|x| *x!=0).collect();
            ret.push(items);
        }
        return;
    }
    
    for i in 0..2 {
        v[depth as usize] = i * (depth + 1);
        dfs(depth+1, k, n, &mut v, &mut ret);
    }
}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut v: Vec<i32> = Vec::new();
        v.resize(9, 0);
        dfs(0, k, n, &mut v, &mut ret);
        ret.sort();
        ret
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::combination_sum3(3, 7), vec![[1, 2, 4]]);
    assert_eq!(Solution::combination_sum3(3, 9), vec![[1,2,6], [1,3,5], [2,3,4]]);
}

