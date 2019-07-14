use std::collections::HashSet;
use std::collections::HashMap;

fn find_leaf(map: &HashMap<i32, (HashSet<i32>, HashSet<i32>)>) -> Option<i32> {
    for (k, v) in map {
        if v.1.is_empty() {
            return Some(*k);
        }
    }
    None
}

fn is_empty(i: i32, map: &HashMap<i32, (HashSet<i32>, HashSet<i32>)>) -> bool {
    let v = map.get(&i).unwrap();
    v.0.is_empty() && v.1.is_empty() 
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // key是节点id, value为(指向当前节点的id列表, 当前节点指向的id列表)
        let mut map:HashMap<i32, (HashSet<i32>, HashSet<i32>)> = HashMap::new();
        
        for i in 0..num_courses {
            map.insert(i, (HashSet::new(), HashSet::new()));
        }
        
        for v in prerequisites {
            map.entry(v[0]).and_modify(|x| {x.0.insert(v[1]);});    
            map.entry(v[1]).and_modify(|x| {x.1.insert(v[0]);});
        }

        while !map.is_empty() {
            if let Some(i) = find_leaf(&map) {
                //println!("find leaf {}", i);
                let from = map.remove(&i).unwrap().0;
                for f in from.iter() {
                    map.entry(*f).and_modify(|x| {
                        x.1.remove(&i);
                    });
                    
                    if is_empty(*f, &map) {
                        map.remove(f);
                    }
                }
            } else {
                return false;
            }
        }
        
        map.is_empty()
    }
}

pub struct Solution;

fn main() {
    println!("{}", Solution::can_finish(2, vec![vec![1,0],vec![0,1]]));
    println!("{}", Solution::can_finish(2, vec![vec![1,0]]));
}