fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    
    let x = (n as f64).sqrt() as i32;
    for i in 2..=x {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut ret = 0;
        for i in 1..n {
            if is_prime(i) {
                ret += 1;
            }
        }
        
        ret
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::count_primes(2), 0);
    assert_eq!(is_prime(2), true);
}