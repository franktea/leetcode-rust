impl Solution {
    // 如果x坐标方向有重叠，且y坐标有重叠，则矩形重叠。
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (x1, y1, x2, y2) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (x3, y3, x4, y4) = (rec2[0], rec2[1], rec2[2], rec2[3]);
        
        // 检查x或者y是否重叠
        fn check(x1: i32, x2: i32, x3: i32, x4: i32) -> bool {
            (x1 == x3 && x2 == x4) // 完全重叠，或者：
                // 一条边在另一个矩形中间，有四种情况
                || (x3 < x1 && x1 < x4) // x1 在 x3,x4中间
                || (x3 < x2 && x2 < x4) // x2 在 x3,x4中间
                || (x1 < x3 && x3 < x2) // x3 在 x1,x2中间
                || (x1 < x4 && x4 < x2) // x4 在 x1,x2中间
        }
        
        check(x1, x2, x3, x4) && check(y1, y2, y3, y4)
    }
}

//end submission
struct Solution;

fn main() {
    
}