impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut ret = 0;
        // 从左到右，计算以第heights[i]为顶的矩形的面积
        for i in 0..heights.len() {
            if heights[i] == 0 {
                continue;
            }
            
            // 计算以heights[i]为顶的矩形的宽度，将相邻的大于heights[i]的个数算出来
            let mut width = 0i32;
            // 左边
            width += (&heights[..i]).iter().rev().take_while(|x| x>=&&heights[i]).count() as i32;
            // 右边
            width += (&heights[i..]).iter().take_while(|x| x>=&&heights[i]).count() as i32;
            
            ret = max(width*heights[i], ret);
        }
        
        ret
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::largest_rectangle_area(vec![2,1,5,6,2,3]));
}