impl Solution {
    // 如果矩形重叠，则最后的面积为两个矩形的面积和减去重叠的面积。
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        // 用(x1,y1)~(x2,y2)表示矩形1，(x3,y3)~(x4,y4)表示矩形2
        let (x1, x2, x3, x4) = (a, c, e, g);
        let (y1, y2, y3, y4) = (b, d, f, h);
        
        // 分别从x方向和y方向判断是否重叠，判断的逻辑一样，写成一个函数
        // 如果有重叠，返回重叠的边长，计算面积只需要边长即可。
        // 如果无重叠，则返回0，计算面积的时候不影响结果
        fn check(mut x1: i32, mut x2: i32, mut x3: i32, mut x4: i32) -> i32 {
            use std::mem::swap;
            // 将坐标值较小的矩形交换为第一个，后面判断起来更简单
            if x3 < x1 {
                swap(&mut x1, &mut x3);
                swap(&mut x2, &mut x4);
            }
            
            // 先返回完全不重叠的情况
            if x3 >= x2 {
                return 0;
            }
            
            use std::cmp::{max, min};
            // 剩下的就是有重叠的情况，重叠边长可以归结为如下算式：
            return min(x2, x4) - max(x1, x3);
        }
        
        (x2-x1)*(y2-y1) + (x4-x3)*(y4-y3) - check(x1,x2,x3,x4)*check(y1,y2,y3,y4)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
}