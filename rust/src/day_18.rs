use itertools::Itertools;
use std::fmt;
use took::Timer;

fn parse(line : &mut Vec<char>) -> Vec<(u8, u8)> {
    let mut result = Vec::new();
    
    let mut depth = 0;
    while line.len() > 0 {
        let next = line.remove(0);
        if next == '[' {
            depth += 1;
        } else if next == ']' {
            depth -= 1;
        } else if next >= '0' && next <= '9' {
            result.push((depth, next.to_digit(10).unwrap() as u8));
        } else {
            //do nothing
        }
    }
    
    return result;
}

fn add(op1 : &Vec<(u8,u8)>, op2 : &Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    return vec![op1.iter().map(|v| (v.0+1, v.1)).collect::<Vec<(u8,u8)>>(), 
                op2.iter().map(|v| (v.0+1, v.1)).collect()].iter().flatten()
        .map(|x| *x)
        .collect();
}

fn explode(op: &mut Vec<(u8, u8)>) -> bool {
    for i in 0 .. op.len() {
        if op[i].0 == 5 {
            if i > 0 {
                //add nearest left
                op[i-1] = (op[i-1].0, op[i-1].1 + op[i].1);
            }
            if i+2 < op.len() {
                //add nearest right
                op[i+2] = (op[i+2].0, op[i+2].1 + op[i+1].1);
            }
            op[i] = (4, 0);
            op.remove(i+1);
            return true;
        }
    }
    return false;
}

fn split(op: &mut Vec<(u8, u8)>) -> bool {
    
    for i in 0 .. op.len()  {
        if op[i].1 > 9 {
            let x = op[i].1;
            let depth = op[i].0 + 1;
            let left = x / 2;
            let right = if x % 2 == 0 { x / 2 } else { 1 + (x/2) };
            op[i] = (depth, left);
            op.insert(i+1, (depth, right));
            return true;
        }
    }
    
    return false;
}

fn magnitude(op: &Vec<(u8, u8)>) -> usize {
    let mut temp : Vec<(u8,usize)> = Vec::new();
    
    let mut i = 0;
    while i < op.len() {
        if i+1 < op.len() && op[i].0 == op[i+1].0 {
            temp.push((op[i].0-1,(op[i].1 * 3 + op[i + 1].1 * 2) as usize));
            i += 2;
        } else {
            temp.push((op[i].0, op[i].1 as usize));
            i += 1;
        }
    }
    
    let mut iters = 0;
    while temp.len() > 1 && iters < 5 {
        let mut i = 0; 
        while i < temp.len() {
            if i+1 < temp.len() && temp[i].0 == temp[i+1].0 {
                let left = temp.remove(i);
                let right = temp.remove(i);
                temp.insert(i,(left.0-1,(left.1 * 3 + right.1 * 2) as usize));
                i += 1;
            } else {
                i += 1;
            }
        }
        iters += 1;
    }
    return temp[0].1;
}

pub fn part_one() {
    let timer = Timer::new();
    let mut input : Vec<_> = include_str!("../inputs/day18.input")
        .lines()
        .map(|x| parse(&mut x.chars().collect_vec())).collect();

    let mut current = input.remove(0);
    
    for i in input {
        current = add(&current, &i);
        while explode(&mut current) || split(&mut current) {};
    }

    let mag = magnitude(&current);
    println!("day 18 part 1: {} in time {}us", mag, timer.took().as_std().as_micros());
}

pub fn part_two() {
    let timer = Timer::new();
    let mut input : Vec<Vec<(u8,u8)>> = include_str!("../inputs/day18.input")
        .lines()
        .map(|x| parse(&mut x.chars().collect_vec())).collect();

    let mut max_mag = 0;
    
    for i in 0 .. input.len() {
        for j in i+1 .. input.len() {
            let mut lhs = input.get(i).unwrap();
            let mut rhs = input.get(j).unwrap();
            let mut current = add(lhs, rhs);
            while explode(&mut current) || split(&mut current) { }
            let mag = magnitude(&current);
            if mag > max_mag { max_mag = mag; }
            
            let mut current = add(rhs, lhs);
            while explode(&mut current) || split(&mut current) { }
            let mag = magnitude(&current);
            if mag > max_mag { max_mag = mag; }
        }
    }
    
    println!("day 18 part 2: {} in time {}ms", max_mag, timer.took().as_std().as_millis());
}