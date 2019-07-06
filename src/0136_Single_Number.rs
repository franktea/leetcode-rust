impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        for i in &nums[1..] {
            ret ^= i;
        }
        ret
    }
}
