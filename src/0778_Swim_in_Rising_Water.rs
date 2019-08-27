use std::cmp::max;

struct Grid {
    grid: Vec<Vec<i32>>,
    visited: Vec<Vec<bool>>,
    ret: i32,
    n: usize,
}

impl Grid {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        Grid {
            n: grid.len(),
            visited: vec![vec![false; grid.len()]; grid.len()], 
            grid,
            ret: 0,
        }
    }
    
    pub fn next(&mut self, y: usize, x: usize) -> Option<(usize, usize)> {
        // 找四个方向里面未访问过且数字最小的
        let mut ret: Option<(usize, usize)> = None;
        let mut min_value = std::i32::MAX;
        // 左
        if x > 0 {
            let x2 = x-1;
            let y2 = y;
            if !self.visited[y2][x2] && self.grid[y2][x2] < min_value {
                min_value = self.grid[y2][x2];
                ret = Some((y2, x2));
            }
        }
        
        // 上
        if y > 0 {
            let x2 = x;
            let y2 = y-1;
            if !self.visited[y2][x2] && self.grid[y2][x2] < min_value {
                min_value = self.grid[y2][x2];
                ret = Some((y2, x2));
            }
        }
        
        // 右
        if x+1 < self.n {
            let x2 = x+1;
            let y2 = y;
            if !self.visited[y2][x2] && self.grid[y2][x2] < min_value {
                min_value = self.grid[y2][x2];
                ret = Some((y2, x2));
            }
        }
        
        // 下
        if y+1 < self.n {
            let x2 = x;
            let y2 = y+1;
            if !self.visited[y2][x2] && self.grid[y2][x2] < min_value {
                min_value = self.grid[y2][x2];
                ret = Some((y2, x2));
            }            
        }
        
        ret
    }
    
    pub fn dfs(&mut self, y: usize, x: usize, max_ele: i32) -> bool {
        println!("visit ({}, {}), e={}", y, x, max_ele);
        self.visited[y][x] = true;
        let max_ele = max(max_ele, self.grid[y][x]);
        
        if y == self.n-1 && x == self.n - 1 {
            self.ret = max_ele;
            return true;
        }
        
        while let Some((y2, x2)) = self.next(y, x) {
            if self.dfs(y2, x2, max_ele) {
                return true;
            }
        }
        
        return false;
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut g = Grid::new(grid);
        g.dfs(0, 0, 0);
        return g.ret;
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::swim_in_water(vec![
        vec![ 0, 1, 2, 3, 4],
        vec![24,23,22,21, 5],
        vec![12,13,14,15,16],
        vec![11,17,18,19,20],
        vec![10, 9, 8, 7, 6]]));
}