impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        let mut num = num;
        for x in [2, 3, 5].iter() {
            while num > 1 && num % x == 0 {
                num /= x;
            }
        }
        num == 1
    }
}