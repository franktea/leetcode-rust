use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Item {
    pre: HashSet<i32>,
    to: HashSet<i32>,
}

impl Item {
    pub fn new() -> Self {
        Item {
            pre: HashSet::new(),
            to: HashSet::new()
        }
    }
    
    pub fn empty(&self) -> bool {
        self.pre.is_empty() && self.to.is_empty()
    }
    
    pub fn delete_to(&mut self, i: i32) {
        self.to.remove(&i);
    }
    
    pub fn is_leaf(&self) -> bool {
        self.to.is_empty()
    }
}

pub fn find_leaf(map: &HashMap<i32, Item>) -> Option<i32> {
    for (k, v) in map.iter() {
        if v.is_leaf() {
            return Some(*k);
        }
    }
    None
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map: HashMap<i32, Item> = HashMap::new();
        
        for i in 0..num_courses {
            map.insert(i, Item::new());
        }
        
        for v in prerequisites {
            map.entry(v[0]).and_modify(|x| { x.to.insert(v[1]); });
            map.entry(v[1]).and_modify(|x| { x.pre.insert(v[0]); });
        }

        let mut ret: Vec<i32> = Vec::new();
        while !map.is_empty() {
            if let Some(i) = find_leaf(&map) {
                ret.push(i);
                let leaf = map.remove(&i).unwrap().pre;
                for n in leaf.iter() {
                    map.entry(*n).and_modify(|x| {
                        x.to.remove(&i);
                    });
                    if map.get(n).unwrap().empty() {
                        map.remove(&n);
                        ret.push(*n);
                    }
                }
            } else {
                return Vec::new();
            }
        }
        ret
    }
}

pub struct Solution;

fn main() {
    println!("{:?}", Solution::find_order(4, vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]]));
}