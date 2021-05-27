impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        if digits.is_empty() {
            return ret;
        }
        
        let chars: Vec<Vec<char>> = vec![vec![], vec![],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z']];
        let ints: Vec<usize> = digits.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
        let mut string: Vec<char> = vec![' '; ints.len()];
        fn dfs(index: usize, ints: &Vec<usize>, chars: &Vec<Vec<char>>, string: &mut Vec<char>, ret: &mut Vec<String>) {
            if index == ints.len() {
                ret.push(string.iter().collect());
                return;
            }
            
            for c in &chars[ints[index]] {
                string[index] = *c;
                dfs(index+1, &ints, &chars, string, ret);
            }
        }
        
        dfs(0, &ints, &chars, &mut string, &mut ret);
        ret
    }
}

// end submission-------------------

struct Solution;

fn main() {
    println!("{:?}", Solution::letter_combinations("234".to_string()));
}