impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        
        use std::cmp::max;
        let mut ret = 0;
        let width = matrix[0].len();
        let height = matrix.len();
        
        for y in 0..height {
            for x in 0..width {
                if matrix[y][x] == '0' {
                    continue;
                }
                
                ret = max(1, ret);
                
                let mut max_x = width;
                let max_y = height;
                
                let mut dy = 0;
                while y + dy < max_y {
                    let mut dx = 0;
                    while x + dx < max_x {
                        if matrix[y+dy][x+dx] == '0' {
                            max_x = x + dx;
                            break;
                        }
                        
                        ret = max(ret, (dy+1)*(dx+1));
                        dx += 1;
                    }
                    dy += 1;
                }
            }
        }
        
        ret as i32
    }
}

// end of submission-------------------------

struct Solution;

fn main() {
    let matrix = vec![
        vec!['1','0','1','0','0'],
        vec!['1','0','1','1','1'],
        vec!['1','1','1','1','1'],
        vec!['1','0','0','1','0'],
    ];
    println!("{}", Solution::maximal_rectangle(matrix));
}