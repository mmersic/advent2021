use took::Timer;
use std::collections::{HashMap};


pub fn part_one() {
    let timer = Timer::new();

    let pairs = include_str!("../inputs/day12.input")
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut G : HashMap<&str, Vec<&str>> = HashMap::new();

    pairs.iter().for_each(|p| {
        if G.contains_key(p.0) {
            G.get_mut(p.0).unwrap().push(p.1);
        } else {
            let v: Vec<&str> = Vec::new();
            G.insert(p.0, v);
            G.get_mut(p.0).unwrap().push(p.1);
        }
        if G.contains_key(p.1) {
            G.get_mut(p.1).unwrap().push(p.0);
        } else {
            let v: Vec<&str> = Vec::new();
            G.insert(p.1, v);
            G.get_mut(p.1).unwrap().push(p.0);
        }

    });

    let mut visited = Vec::from(["start"]);

    println!("day 12, part 1: {} in time {}us", paths(&G, "start", &mut visited), timer.took().as_std().as_micros());
}

pub fn paths<'a>(G : &HashMap<&str, Vec<&'a str>>, current : &str, visited : &mut Vec<&'a str>) -> usize {
    let mut path_count : usize = 0;
    G.get(current).unwrap()
        .iter()
        .for_each(|&next| {
            if next == "end" {
                path_count += 1;
            } else if next.as_bytes()[0] > b'Z' && visited.contains(&next) {
                //duplicate small cave, don't go further, don't count this path
            } else {
                visited.push(next);
                path_count += paths(G, next, visited);
                visited.pop();
            }
        });
    return path_count;
}



pub fn part_two() {
    let timer = Timer::new();

    let pairs = include_str!("../inputs/day12.input")
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut G : HashMap<&str, Vec<&str>> = HashMap::new();

    pairs.iter().for_each(|p| {
        if G.contains_key(p.0) {
            G.get_mut(p.0).unwrap().push(p.1);
        } else {
            let v: Vec<&str> = Vec::new();
            G.insert(p.0, v);
            G.get_mut(p.0).unwrap().push(p.1);
        }
        if G.contains_key(p.1) {
            G.get_mut(p.1).unwrap().push(p.0);
        } else {
            let v: Vec<&str> = Vec::new();
            G.insert(p.1, v);
            G.get_mut(p.1).unwrap().push(p.0);
        }

    });

    let mut visited = Vec::from(["start"]);

    println!("day 12, part 2: {} in time {}us", paths_two(&G, "start", &mut visited, false), timer.took().as_std().as_micros());
}

pub fn paths_two<'a>(G : &HashMap<&str, Vec<&'a str>>, current : &str, visited : &mut Vec<&'a str>, twice : bool) -> usize {
    let mut path_count : usize = 0;
    G.get(current).unwrap()
        .iter()
        .for_each(|&next| {
            if next == "end" {
                path_count += 1;
            } else if next == "start" {
                //not allowed, don't go further, don't count this path.
            } else if next.as_bytes()[0] > b'Z' && visited.contains(&next) {
                if twice {
                    //duplicate small cave, don't go further, don't count this path.
                } else {
                    path_count += paths_two(G, next, visited, true);
                }
            } else {
                visited.push(next);
                path_count += paths_two(G, next, visited, twice);
                visited.pop();
            }
        });
    return path_count;
}
