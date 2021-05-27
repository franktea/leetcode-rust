impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        fn find_next_not_of(nums: &Vec<i32>, v: i32, p: usize) -> usize {
            let mut p2 = p;
            while p2 < nums.len() && nums[p2] == v {
                p2 += 1;
            }
            
            return p2;
        }
        
        let mut v = nums[0];
        let mut p1 = 1;
        let mut p2 = 1;
        while p1 < nums.len() {
            p2 = find_next_not_of(nums, v, p2);
            if p2 >= nums.len() {
                break;
            }
            nums.swap(p1, p2);
            v = nums[p1];
            p1 += 1;
            p2 += 1;
        }
        
        p1 as i32
    }
}

// end submission

struct Solution;

fn main() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];
    println!("{}", Solution::remove_duplicates(&mut v));
    println!("{:?}", &v);
}