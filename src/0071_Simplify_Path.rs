// 直接用栈就可以了
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut v = Vec::new();
        for s in path.split("/") {
            match s {
                "" | "." => {
                    
                }
                ".." => {
                    v.pop();
                }
                _ => {
                    v.push(s);
                }
            }
        }
        
        if v.is_empty() {
            return "/".to_string();
        }
        
        "/".to_string() + &v.join("/")
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo");
    assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c");
    assert_eq!(Solution::simplify_path("/a/../../b/../c//.//".to_string()), "/c");
    
    assert_eq!(Solution::simplify_path("/a//b////c/d//././/..".to_string()), "/a/b/c");
}