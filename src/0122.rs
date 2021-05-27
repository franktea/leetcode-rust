fn valley(i: usize, v: &Vec<i32>) -> Option<usize> {
    let mut i = i;
    if i == v.len() - 1 {
        return None;
    }
    while i < v.len() - 1 && v[i] >= v[i+1] {
        i += 1;
    }
    Some(i)
}

fn peak(i: usize, v: &Vec<i32>) -> Option<usize> {
    let mut i = i;
    if i == v.len() - 1 {
        return None;
    }
    while i < v.len() - 1 && v[i] <= v[i+1] {
        i += 1;
    }
    Some(i)
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        
        let mut ret = 0;
        let mut i = 0;
        loop {
            let v = valley(i, &prices);
            match v {
                None => { break; }
                Some(index) => { i = index; }
            }
            let p = peak(i, &prices);
            match p {
                None => { break; }
                Some(index) => { i = index; }
            }
            ret += prices[p.unwrap()] - prices[v.unwrap()];
        }
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}", Solution::max_profit(vec![1,2,3,4,5]));
}