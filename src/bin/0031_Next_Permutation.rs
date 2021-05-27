fn reverse(nums: &mut Vec<i32>, i: usize, j: usize) {
    let mut i = i;
    let mut j = j;
    while i < j {
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }
}

// 照着stl的next_premutation改的，c++代码：
// https://stackoverflow.com/questions/11483060/stdnext-permutation-implementation-explanation
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        
        let mut i = nums.len() - 1;
        loop {
            let j = i;
            i = j - 1;
            
            if nums[i] < nums[j] {
                let mut k = nums.len() - 1;
                while ! (nums[i] < nums[k]) {
                    k -= 1;
                }
                
                nums.swap(i, k);
                reverse(nums, j, nums.len() - 1);
                return;
            }
            
            if i == 0 {
                reverse(nums, 0, nums.len() - 1);
                return;
            }
        }
    }
}

// end submission------------
struct Solution;

fn main() {
    let mut v = vec![1i32, 2, 3];
    Solution::next_permutation(&mut v);
    assert_eq!(v, [1, 3, 2]);
    
    let mut v = vec![3i32, 2, 1];
    Solution::next_permutation(&mut v);
    assert_eq!(v, [1, 2, 3]);
    
    let mut v = vec![1i32, 1, 5];
    Solution::next_permutation(&mut v);
    assert_eq!(v, [1, 5, 1]);
    
    let mut v = vec![1i32, 5, 1];
    Solution::next_permutation(&mut v);
    assert_eq!(v, [5, 1, 1]);    
}