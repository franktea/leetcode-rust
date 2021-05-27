
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        for i in 0..16 {
            let n = i*2;
            if num == (1 << n) {
                return true;
            }
        }
        false
    }
}