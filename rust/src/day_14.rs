use std::cmp::Ordering;
use took::Timer;
use std::collections::{HashMap};
use std::str;

pub fn part_one() {
    let timer = Timer::new();

    let (start_str, rules_str) = include_str!("../inputs/day14.sample")
        .split_once("\n\n").unwrap();
    
    let mut current = start_str.as_bytes().to_vec();
    
    let rules : HashMap<_,_> = rules_str.lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .fold(HashMap::new(), |mut acc, (lhs, rhs)| {
           acc.insert(lhs, rhs);
            acc
        });

    let iters = 10;
    
    for _i in 0 .. iters {
        let mut j = 1;
        loop {
            let pair = [current[j - 1], current[j]];
            
            let current_pair = str::from_utf8(pair.as_slice()).unwrap();
            
            current.insert(j, rules.get(current_pair).unwrap().as_bytes()[0]);
            
            j += 2;
            if j >= current.len() {
                break;
            }
        }
        // println!("iter {}: {}", i, str::from_utf8(&current).unwrap());
    }
    
    let map: HashMap<_, usize> = current.iter().fold(HashMap::new(), |mut acc, x| {
        let new_val = *acc.entry(x).or_default()+1;
       acc.insert(x, new_val);
        acc
    });
    
    let max = map.iter().max_by(|a, b| if a.1 > b.1 {Ordering::Greater} else {Ordering::Less}).unwrap().1;
    let min = map.iter().min_by(|a, b| if a.1 > b.1 {Ordering::Greater} else {Ordering::Less}).unwrap().1;
    
    println!("day 14, part 1: {} in time {}us", max-min, timer.took().as_std().as_micros());
}


pub fn part_two() {
    let timer = Timer::new();

    let (start_str, rules_str) = include_str!("../inputs/day14.input")
        .split_once("\n\n").unwrap();

    let current = start_str.as_bytes().to_vec();

    let rules : HashMap<(u8, u8),u8> = rules_str.lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .fold(HashMap::new(), |mut acc, (lhs, rhs)| {
            let b = lhs.as_bytes();
            acc.insert((b[0], b[1]), rhs.as_bytes()[0]);
            acc
        });
    
    let mut map : HashMap<(u8, u8, usize), HashMap<u8, usize>> = HashMap::new();

    let iters = 40;
    
    for i in 1 .. current.len() {
        count_letters((current[i-1], current[i], iters), &mut map, &rules);
    }
    
    let mut result : HashMap<u8, usize> = HashMap::new();
    
    for i in 1 .. current.len() {
        let m1 = map.get(&(current[i-1], current[i], iters)).unwrap();

        if i > 1 {
            result.insert(current[i-1], result.get(&current[i-1]).unwrap()-1); //so far, we double counted middle char, so subtract 1
        }
        
        m1.iter().for_each(|x| {
            if result.contains_key(x.0) {
                result.insert(*x.0, result.get(x.0).unwrap() + x.1);
            } else {
                result.insert(*x.0, *x.1);
            }
        });
    }

    let max = result.iter().max_by(|a, b| if a.1 > b.1 {Ordering::Greater} else {Ordering::Less}).unwrap().1;
    let min = result.iter().min_by(|a, b| if a.1 > b.1 {Ordering::Greater} else {Ordering::Less}).unwrap().1;

    println!("day 14, part 2: {} in time {}us", max-min, timer.took().as_std().as_micros());
}

fn count_letters(current : (u8, u8, usize), map : &mut HashMap<(u8, u8, usize), HashMap<u8, usize>>, rules : &HashMap<(u8, u8),u8>) {
    if map.contains_key(&current) {
        return;
    } else if current.2 == 0 {
        let mut some_map : HashMap<u8, usize> = HashMap::new();
        if current.0 == current.1 {
            some_map.insert(current.0, 2);
        } else {
            some_map.insert(current.0, 1);
            some_map.insert(current.1, 1);
        }
        map.insert((current.0, current.1, 0), some_map);
    } else {
        let rule = rules.get(&(current.0, current.1)).unwrap();
        count_letters((current.0, *rule, current.2-1), map, rules);
        count_letters((*rule, current.1, current.2-1), map, rules);
        
        let m1 = map.get(&(current.0, *rule, current.2-1)).unwrap();
        let m2 = map.get(&(*rule, current.1, current.2-1)).unwrap();
        
        let mut  merged : HashMap<u8, usize> = HashMap::new();
        
        m1.iter().for_each(|x| {
           if merged.contains_key(x.0) {
               let v = merged.get(x.0).unwrap() + x.1;
               merged.insert(*x.0, v);
           } else {
               merged.insert(*x.0, *x.1);
           }
        });
        m2.iter().for_each(|x| {
            if merged.contains_key(x.0) {
                let v = merged.get(x.0).unwrap() + x.1;
                merged.insert(*x.0, v);
            } else {
                merged.insert(*x.0, *x.1);
            }
        });
        
        merged.insert(*rule, merged.get(rule).unwrap()-1); //so far, we double counted rule, so subtract 1
        
        map.insert((current.0, current.1, current.2), merged);
    }
}



pub fn part_two_optimized() {
    let timer = Timer::new();

    let (start_str, rules_str) = include_str!("../inputs/day14.input")
        .split_once("\n\n").unwrap();

    let current = start_str.as_bytes().to_vec();

    let rules : HashMap<(u8, u8),u8> = rules_str.lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .fold(HashMap::new(), |mut acc, (lhs, rhs)| {
            let b = lhs.as_bytes();
            acc.insert((b[0], b[1]), rhs.as_bytes()[0]);
            acc
        });

    let mut map : HashMap<(u8, u8, usize), HashMap<u8, usize>> = HashMap::new();

    let iters = 40;

    for i in 1 .. current.len() {
        count_letters((current[i-1], current[i], iters), &mut map, &rules);
    }

    let mut result : HashMap<u8, usize> = HashMap::new();

    for i in 1 .. current.len() {
        let m1 = map.get(&(current[i-1], current[i], iters)).unwrap();

        if i > 1 {
            result.insert(current[i-1], result.get(&current[i-1]).unwrap()-1); //so far, we double counted middle char, so subtract 1
        }

        m1.iter().for_each(|x| {
            if result.contains_key(x.0) {
                result.insert(*x.0, result.get(x.0).unwrap() + x.1);
            } else {
                result.insert(*x.0, *x.1);
            }
        });
    }

    let max = result.iter().max_by(|a, b| if a.1 > b.1 {Ordering::Greater} else {Ordering::Less}).unwrap().1;
    let min = result.iter().min_by(|a, b| if a.1 > b.1 {Ordering::Greater} else {Ordering::Less}).unwrap().1;

    println!("day 14, part 2: {} in time {}us", max-min, timer.took().as_std().as_micros());
}