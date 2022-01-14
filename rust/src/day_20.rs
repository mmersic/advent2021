use took::Timer;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn calc_entry(p1 : &[u8], map : &HashMap<(i32, i32), u8>, x : i32, y : i32, iter : i32) -> usize {
    let mut temp = 0;
    
    let inf = p1.get(if iter % 2 == 0 { 0 } else { 511 }).unwrap();
    
    let dot = &b'.';

    temp += if map.get(&(x-1, y-1)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x, y-1)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x+1, y-1)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x-1, y)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x, y)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x+1, y)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x-1, y+1)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x, y+1)).unwrap_or(inf) == dot { 0 } else { 1 };
    temp *= 2;
    temp += if map.get(&(x+1, y+1)).unwrap_or(inf) == dot { 0 } else { 1 };
    
    return temp;
}

pub fn enhance(p1 : &[u8], map : &HashMap<(i32, i32), u8>, iter : i32) -> HashMap<(i32, i32),u8> {
    let mut next : HashMap<(i32, i32), u8> = HashMap::new();
    
    for x in (0 - iter) as i32 .. (100 + iter) as i32 {
        for y in (0 - iter) as i32 .. (100 + iter) as i32 {
            next.insert((x, y), *p1.get(calc_entry(p1, map, x, y, iter)).unwrap());
        }
    }
    
    return next;
}

pub fn part_one() {
    let timer = Timer::new();
    let (i1, i2) = include_str!("../inputs/day20.input")
        .split_once("\n\n").unwrap();
    
    let mut map : HashMap<(i32, i32), u8> = HashMap::new();
    
    let dot = &b'.';
    let hash = &b'#';
    
    let p1 = i1.as_bytes();
    let p2 = i2.split("\n").map(|x| x.as_bytes()).collect_vec();
    
    for y in 0 .. p2.len() {
        let row = p2.get(y).unwrap();
        for x in 0 .. row.len() {
            map.insert((x as i32, y as i32), *row.get(x).unwrap());
        }
    }

    for y in 0 as i32 .. p2.len() as i32 {
        for x in 0 as i32 .. p2.get(0).unwrap().len() as i32 {
            print!("{}", if dot == map.get(&(x, y)).unwrap() { "." } else { "#" });
        }
        println!();
    }
    
    println!("map count: {}", map.len());
    let lit_count = map.iter().fold(0, |acc, x| acc + if x.1 == hash { 1 } else { 0 });
    println!("lit count: {}", lit_count);
    

    let next = enhance(p1, &map, 1);
    
    println!("map count: {}", next.len());
    let lit_count = next.iter().fold(0, |acc, x| acc + if x.1 == hash { 1 } else { 0 });
    println!("lit count: {}", lit_count);

    for y in -1 as i32 .. (p2.len()+1) as i32 {
        for x in -1 as i32 .. (p2.get(0).unwrap().len()+1) as i32 {
            print!("{}", if dot == next.get(&(x, y)).unwrap() { "." } else { "#" });
        }
        println!();
    }
    
    
    let next = enhance(p1, &next, 2);
    println!("map count: {}", next.len());
    let lit_count = next.iter().fold(0, |acc, x| acc + if x.1 == hash { 1 } else { 0 });
    println!("lit count: {}", lit_count);

    for y in -2 as i32 .. (p2.len()+2) as i32 {
        for x in -2 as i32 .. (p2.get(0).unwrap().len()+2) as i32 {
            print!("{}", if dot == next.get(&(x, y)).unwrap() { "." } else { "#" });
        }
        println!();
    }


    println!("day 20 part 1: {} in time {}us", lit_count, timer.took().as_std().as_micros());

}

pub fn part_two() {
    let timer = Timer::new();
    let (i1, i2) = include_str!("../inputs/day20.input")
        .split_once("\n\n").unwrap();

    let mut map : HashMap<(i32, i32), u8> = HashMap::new();

    let dot = &b'.';
    let hash = &b'#';

    let p1 = i1.as_bytes();
    let p2 = i2.split("\n").map(|x| x.as_bytes()).collect_vec();

    for y in 0 .. p2.len() {
        let row = p2.get(y).unwrap();
        for x in 0 .. row.len() {
            map.insert((x as i32, y as i32), *row.get(x).unwrap());
        }
    }

    for y in 0 as i32 .. p2.len() as i32 {
        for x in 0 as i32 .. p2.get(0).unwrap().len() as i32 {
            print!("{}", if dot == map.get(&(x, y)).unwrap() { "." } else { "#" });
        }
        println!();
    }

    println!("map count: {}", map.len());
    let lit_count = map.iter().fold(0, |acc, x| acc + if x.1 == hash { 1 } else { 0 });
    println!("lit count: {}", lit_count);
    
    let mut next = map;
    
    for x in 1 .. 51 {
        next = enhance(p1, &next, x);
    }

    let lit_count = next.iter().fold(0, |acc, x| acc + if x.1 == hash { 1 } else { 0 });
    println!("lit count: {}", lit_count);

    for y in -50 as i32 .. (p2.len()+50) as i32 {
        for x in -50 as i32 .. (p2.get(0).unwrap().len()+50) as i32 {
            print!("{}", if dot == next.get(&(x, y)).unwrap() { " " } else { "#" });
        }
        println!();
    }


    println!("day 20 part 1: {} in time {}us", lit_count, timer.took().as_std().as_micros());}