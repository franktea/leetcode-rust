// 这题没那么轻松！！
impl Solution {
    fn check(nums: &Vec<i32>, i: usize) -> bool {
        let a = &nums[..i];
        let b = &nums[i+1..];
        let increase = |x: &[i32]| -> bool {            
            let mut index = 0;
            while index < x.len() - 1 && x[index] <= x[index+1] {
                index += 1;
            }
            return index == x.len() - 1;
        };
        
        return increase(a) && increase(b) && a.last().unwrap() <= &b[0];
    }
    
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return true;
        }
        
        let mut nums = nums;
        nums.insert(0, std::i32::MIN);
        nums.push(std::i32::MAX);
        
        let mut i = 0;
        while i < nums.len() - 1 && nums[i] <= nums[i+1] {
            i += 1;
        }
        
        if i == nums.len() - 1 {
            return true;
        }
        
        return Solution::check(&nums, i) || Solution::check(&nums, i+1);
    }
}

pub struct Solution;

fn main() {
    assert!(Solution::check_possibility(vec![1, 2, 3]));
    assert!(Solution::check_possibility(vec![1, 2]));
    assert!(!Solution::check_possibility(vec![4, 2, 1]));
    assert!(Solution::check_possibility(vec![4, 2, 3]));
    assert!(Solution::check_possibility(vec![2,3,3,2,4]));
    assert!(Solution::check_possibility(vec![1,3,2]));
    assert!(Solution::check_possibility(vec![-1,4,2,3]));
}