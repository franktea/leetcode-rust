impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m;
        let mut index:usize = 0;
        for i in 0..(n as usize) {
            while (index < m as usize) && nums1[index] <= nums2[i] {
                index += 1;
            }
            
            if index < (m as usize) {
                for j in (index+1..nums1.len()).rev() {
                    nums1[j] = nums1[j-1];
                }
                m += 1;
            }
            nums1[index] = nums2[i];
            index += 1;
        }
    }
}

pub struct Solution;

fn main() {
    {
        let v1 = &mut vec![1,2,3,0,0,0];
        Solution::merge(v1, 3, &mut vec![2,5,6], 3);
        assert_eq!(v1, &mut vec![1,2,2,3,5,6]);
    }
    {
        let v1 = &mut vec![-1,0,0,3,3,3,0,0,0];
        Solution::merge(v1, 6, &mut vec![1,2,2], 3);
        assert_eq!(v1, &mut vec![-1,0,0,1,2,2,3,3,3]);
    }
}