pub fn visit(y: usize, x: usize, board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
    visited[y][x] = true;
    let array: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for (dy, dx) in array.iter() {
        let y2 = y as i32 + dy;
        let x2 = x as i32 + dx;
        if y2 >= 0 && y2 < board.len() as i32 && x2 >= 0 && x2 < board[0].len() as i32
            && !visited[y2 as usize][x2 as usize] && board[y2 as usize][x2 as usize] == 'O' {
            visit(y2 as usize, x2 as usize, board, visited);
        }
    }
}

pub fn travel(board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>)
{
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if (y == 0 || x == 0 || y == board.len()-1 || x == board[0].len() - 1) &&
                board[y][x] == 'O' {
                visit(y, x, board, visited);
            }
        }
    }
}

// 将周围的一圈O及其可到达的区域全部标记出来，剩下未标记的全部换成X即可。
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut visited: Vec<Vec<bool>> = Vec::new();
        for v in board.iter() {
            let mut i: Vec<bool> = Vec::new();
            i.resize(v.len(), false);
            visited.push(i);
        }
        
        travel(board, &mut visited);
        for y in 0..visited.len() {
            for x in 0..visited[0].len() {
                if !visited[y][x] {
                    board[y][x] = 'X';
                }
            }
        }
    }
}

pub struct Solution;

fn main() {
    let mut v = vec![vec!['X','X','X','X'],vec!['X','O','O','X'],vec!['X','X','O','X'],vec!['X','O','X','X']];
    Solution::solve(&mut v);
    println!("{:?}", &v);
}