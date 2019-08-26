impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        use std::cmp::{max, min};
        let w = matrix[0].len();
        let h = matrix.len();
        let mut ret = 0;
        for y in 0..h {
            for x in 0..w {
                // 对于以每个点为左上角的所有可能的正方形，一个一个地判断
                // i为正方形的边长，从小到大进行判断，小的已经判断过了，
                // 判断大的正方形的时候只需要判断最右边一条边和最下面一条边，
                // 这样已经在一定程度上减少了判断的次数
                for i in 0..min(h-y, w-x) {
                    let mut finished = false; // 遇到不符合全部是正方形的条件，提前结束标识
                    for index in 0..=i {
                        if matrix[y+i][x+index] != '1' || matrix[y+index][x+i] != '1' {
                            finished = true;
                            break;
                        }
                    }
                    
                    if finished {
                        break;
                    }
                    
                    ret = max(ret, (i+1)*(i+1));
                }
            }
        }
        ret as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximal_square(vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0']]));
}