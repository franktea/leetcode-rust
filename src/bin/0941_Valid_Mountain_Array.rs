impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 { 
            return false;
        }
        
        let mut i = 0;
        while i < a.len() - 1 && a[i] < a[i+1] {
            i += 1;
        }
        
        if i == 0 || i == a.len() -1 {
            return false;
        }
        
        while i < a.len() - 1 && a[i] > a[i+1] {
            i += 1;
        }
        
        return i == a.len() - 1;
    }
}
