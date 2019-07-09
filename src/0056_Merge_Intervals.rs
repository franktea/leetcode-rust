#[derive(Debug)]
struct Item {
    value: i32, // 某个点的坐标
    starts: Vec<Vec<i32>>, // 所有以该坐标为起点的区间
    ends: Vec<Vec<i32>>, // 所有以该坐标为结束点的区间
}

impl Item {
    pub fn new(value: i32, start: bool, v: Vec<i32>) -> Self { 
        if start {
            Item {
                value: value,
                starts: vec![v],
                ends: vec![],
            }
        } else {
            Item {
                value: value,
                starts: vec![],
                ends: vec![v],
            }
        }
    }   
}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, Item> = HashMap::new();
        for v in intervals.iter() {
            map.entry(v[0]).and_modify(|i| i.starts.push(v.clone())).or_insert(Item::new(v[0], true, v.clone()));
            map.entry(v[1]).and_modify(|i| i.ends.push(v.clone())).or_insert(Item::new(v[1], false, v.clone()));
        }
        //println!("{:?}", map);
        
        let mut points: Vec<&Item> = map.values().collect();
        points.sort_by(|x, y| x.value.cmp(&y.value));
        //println!("{:?}", &points);
               
        let mut ret = Vec::new();
        if points.is_empty() {
            return ret;
        }
        
        let mut status = 0; // 0: expect start, 1: expept end
        let mut start = 0;
        let mut end = 0;
        for item in points.iter() {
            if !item.starts.is_empty() {
                if status == 0 {
                    start = item.value;
                    end = start;
                    status = 1;
                }
                let mut max = item.starts[0][1];
                for range in item.starts.iter() {
                    if range[1] > max {
                        max = range[1];
                    }
                }
                if max > end {
                    end = max;
                }
            }
             
            if !item.ends.is_empty() { // item.ends is not empty
                if item.value == end {
                    ret.push(vec![start, end]);
                    status = 0;
                }
            }
        }

        ret
    }
}

pub struct Solution;

fn main() {
    assert_eq!(Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), vec![[1,6],[8,10],[15,18]]);
    assert_eq!(Solution::merge(vec![vec![1,4],vec![4,5]]), vec![[1,5]]);
    assert_eq!(Solution::merge(vec![vec![1,4],vec![0,4]]), vec![[0,4]]);
    assert_eq!(Solution::merge(vec![vec![1,4],vec![0,0]]), vec![[0,0], [1, 4]]);
}
