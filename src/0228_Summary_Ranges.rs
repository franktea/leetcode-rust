impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        if nums.is_empty() {
            return ret;
        }
        
        // 找到下一个不连续的位置，返回值指向该位置的下一个字符
        fn find_next_not_of(nums: &Vec<i32>, from: usize) -> usize {
            let mut v = nums[from];
            let mut p = from+1;
            while p < nums.len() && nums[p] == v + 1 {
                p += 1;
                v += 1;
            }
            
            p
        }
        
        let mut p1 = 0;
        while p1 < nums.len() {
            let p2 = find_next_not_of(&nums, p1);
            if p2 - p1 == 1 {
                ret.push(format!("{}", nums[p1]));
            } else {
                ret.push(format!("{}->{}", nums[p1], nums[p2-1]));
            }
            p1 = p2;
        }
        
        ret
    }
}

// end submission
struct Solution;

fn main() {
    println!("{:?}", Solution::summary_ranges(vec![0,1,2,4,5,7]));
    println!("{:?}", Solution::summary_ranges(vec![0,2,3,4,6,8,9]));
}