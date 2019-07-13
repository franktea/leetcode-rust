impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        //use std::mem::swap;
        let n = matrix.len();
        for y in 0..=(n-1)/2 {
            for x in y..n-1-y {
                //println!("{}, {}", y, x);
                let tmp = matrix[y][x];
                matrix[y][x] = matrix[n-x-1][y];
                matrix[n-x-1][y] = matrix[n-1-y][n-x-1];
                matrix[n-1-y][n-x-1] = matrix[x][n-y-1];
                matrix[x][n-y-1] = tmp;
                //swap(matrix[y][x], matrix[x][n-y-1]);
                //swap(matrix[x][n-y-1], matrix[n-y][n-x-1]);
                //swap(matrix[n-y][n-x-1], matrix[n-x-1][y]);
                //swap(matrix[n-x-1][y], matrix[y][x]);
            }
        }
    }
}

pub struct Solution;

fn main() {
    let v = &mut vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    Solution::rotate(v);
    println!("{:?}", v);
    let mut v = vec![vec![ 5, 1, 9,11], vec![ 2, 4, 8,10], vec![13, 3, 6, 7], vec![15,14,12,16]];
    Solution::rotate(&mut v);
    println!("{:?}", v);
}