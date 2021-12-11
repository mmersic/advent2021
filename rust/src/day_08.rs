use std::cmp::Ordering;
use std::collections::{HashSet};
use took::Timer;

pub fn part_one() {
    let timer = Timer::new();
    let nums = include_str!("../inputs/day8.input")
        .lines();
    
    let result = nums.into_iter()
        .map( |x| x.split(" | ").collect::<Vec<&str>>()[1])
        .into_iter()
        .map( |x | x.split(" ").collect::<Vec<&str>>())
        .flatten()
        .filter( | x | x.len() == 3 || x.len() == 4 || x.len() == 7 || x.len() == 2)
        .count();
    
    println!("day 8, part 1: {} in time {}us", result, timer.took().as_std().as_micros());
}

pub fn part_one_optimized() {
    let timer = Timer::new();
    let result = include_str!("../inputs/day8.input")
        .lines()
        .map( |x| x.split(" | ").collect::<Vec<&str>>()[1].split(" "))
        .flatten()
        .filter( | x | x.len() == 3 || x.len() == 4 || x.len() == 7 || x.len() == 2)
        .count();
    
    println!("day 8, part 1: {} in time {}us", result, timer.took().as_std().as_micros());
}

pub fn part_one_final() {
    let timer = Timer::new();
    let result = include_str!("../inputs/day8.input")
        .lines()
        .flat_map( |x| x.split_once(" | ").unwrap().1.split(" "))
        .filter( | x | matches!(x.len(), 2 | 3 | 4 | 7))
        .count();

    println!("day 8, part 1: {} in time {}us", result, timer.took().as_std().as_micros());
}


fn decode(line : &str) -> usize {
    let a : Vec<&str> = line.split(" | ").collect::<Vec<&str>>();
    
    let signals = a[0].split(" ").map(|x| x.as_bytes().iter().collect::<HashSet<&u8>>()).collect::<Vec<HashSet<&u8>>>();
    let outputs = a[1].split(" ").map(|x| x.as_bytes().iter().collect::<HashSet<&u8>>()).collect::<Vec<HashSet<&u8>>>();

    let one = signals.iter().find(|x| x.len() == 2).unwrap();   
    let seven = signals.iter().find(|x| x.len() == 3).unwrap();
    let eight = signals.iter().find(|x| x.len() == 7).unwrap();
    let four = signals.iter().find(|x| x.len() == 4).unwrap();
    let nine = signals.iter().find(|x| x.len() == 6 && x.is_superset(seven) && x.is_superset(four)).unwrap();
    let six = signals.iter().filter(|x| !x.is_superset(one)).max_by(|a, b| if a.len() > b.len() { Ordering::Greater } else { Ordering::Less }).unwrap();
    let zero = signals.iter().filter(|x| x.len() == 6)
        .find(|x| !x.is_superset(six) && !x.is_superset(nine)).unwrap();
    let five = signals.iter().find(|x| x.len() == 5 && nine.is_superset(x)  && !one.is_subset(x)).unwrap();
    let three = signals.iter().find(|x| x.len() == 5 && x.is_superset(one)).unwrap();
    let two = signals.iter().find(|x| x.len() == 5 && !x.is_superset(five) && !x.is_superset(three)).unwrap();

    let mut result_str : String = String::from("");
    
    for x in outputs {
        if x.eq(zero) {
            result_str += "0";
        } else if x.eq(one) {
            result_str += "1";
        } else if x.eq(two) {
            result_str += "2";
        } else if x.eq(three) {
            result_str += "3";
        } else if x.eq(four) {
            result_str += "4";
        } else if x.eq(five) {
            result_str += "5";
        } else if x.eq(six) {
            result_str += "6";
        } else if x.eq(seven) {
            result_str += "7";
        } else if x.eq(eight) {
            result_str += "8";
        } else if x.eq(nine) {
            result_str += "9";
        } 
    }
    
    return result_str.parse::<usize>().unwrap();
}

pub fn part_two() {
    let timer = Timer::new();
    let nums = include_str!("../inputs/day8.input")
        .lines().collect::<Vec<&str>>();

    let result : usize = nums.iter()
        .map( |x| decode(&x))
        .sum();

    println!("day 8, part 2: {} in time: {}us", result, timer.took().as_std().as_micros());
}

fn decode_optimized(_line : &str) -> usize {
    return 1;
}

pub fn part_two_optimized() {
    let timer = Timer::new();
    let nums = include_str!("../inputs/day8.single")
        .lines().collect::<Vec<&str>>();

    let result : usize = nums.iter()
        .map( |x| decode_optimized(&x))
        .sum();

    println!("day 8, part 2: {} in time: {}us", result, timer.took().as_std().as_micros());
}