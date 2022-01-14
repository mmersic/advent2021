use itertools::Itertools;
use took::Timer;
use std::collections::{HashSet};

pub fn transform(x : &(i32, i32, i32), t : usize) -> (i32, i32, i32) {
    return match t {
        //facing x
        0 => (x.2,x.0,x.1),
        1 => (-x.2,x.0,-x.1),
        2 => (-x.1,x.0,x.2),
        3 => (x.1,x.0,-x.2),
        
        //facing -x
        4 => (-x.2,-x.0,x.1),
        5 => (x.2,-x.0,-x.1),
        6 => (x.1,-x.0,x.2),
        7 => (-x.1,-x.0,-x.2),
        
        //facing z
        8 => (x.1,x.2,x.0),
        9 => (-x.1,x.2,-x.0),
        10 => (-x.0,x.2,x.1),
        11 => (x.0,x.2,-x.1),
        
        //facing -z
        12 => (-x.1,-x.2,x.0),
        13 => (x.1,-x.2,-x.0),
        14 => (x.0,-x.2,x.1),
        15 => (-x.0,-x.2,-x.1),
        
        //facing y
        16 => (-x.2,x.1,x.0),
        17 => (x.2,x.1,-x.0),
        18 => (x.0,x.1,x.2),
        19 => (-x.0,x.1,-x.2),
        
        //facing -y
        20 => (x.2,-x.1,x.0),
        21 => (-x.2,-x.1,-x.0),
        22 => (-x.0,-x.1,x.2),
        23 => (x.0,-x.1,-x.2),
        
        _ => unreachable!()
    }
}

fn transform_by(t : usize, scanner : &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut result : Vec<(i32, i32, i32)> = Vec::new();
    
    for v in scanner {
        result.push(transform(v, t));
    }
    
    return result;
}

fn shift_by(shift_amt : (i32, i32, i32), scanner : &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut result : Vec<(i32, i32, i32)> = Vec::new();
    
    scanner.iter().for_each(|x| {
        let xx = x.0 + shift_amt.0;
        let yy = x.1 + shift_amt.1;
        let zz = x.2 + shift_amt.2;
        result.push((xx, yy, zz));
    });

    return result;
}

fn overlap_pair_scanner(s1 : &((i32, i32, i32), Vec<(i32, i32, i32)>), s2 : &Vec<(i32, i32, i32)>) -> Option<((i32, i32, i32), Vec<(i32, i32, i32)>)> {
    let mut s1_set : HashSet<(i32, i32, i32)> = HashSet::new();

    for p in &s1.1 {
        s1_set.insert(*p);
    }

    for i in 0 .. s1.1.len() {
        let current = s1.1.get(i).unwrap();
        for t in 0 .. 24 {
            for j in 0 .. s2.len() {
                let temp = transform(s2.get(j).unwrap(), t);
                let mut x_shift = current.0 - temp.0;
                let mut y_shift = current.1 - temp.1;
                let mut z_shift = current.2 - temp.2;
                let transformed = transform_by(t, s2);
                let shifted = shift_by((x_shift, y_shift, z_shift), &transformed);
                let mut overlap_count = 0;
                for k in 0 .. shifted.len() {
                    let next = shifted.get(k).unwrap();
                    if s1_set.contains(next) {
                        overlap_count += 1;
                        if overlap_count > 11 {
                            return Some(((x_shift, y_shift, z_shift), shifted));
                        }
                    }
                }
            }
        }
    }


    return None;
}

fn overlap_scanner(oriented: &mut Vec<((i32, i32, i32), Vec<(i32, i32, i32)>)>, input: &mut Vec<Vec<(i32, i32, i32)>>) {

    let known_pairs = [(0,1), (0,1), (0, 19), (0, 28), (1, 1), (1, 9), (3, 22), (3, 24), (5, 22), (6, 4),
        (6,10), (6,20), (7,17), (7,18), (8,9), (8,17), (9, 13), (10, 4), (10, 9), (12, 12), (13, 6),
        (15, 9), (15, 9), (16, 0), (16, 7), (17, 9), (18, 4), (18, 5), (22, 5), (24, 2), (24, 4), (25, 1),
        (25, 1), (29, 0), (29, 0)
    ];

    for p in known_pairs {
        let next_oriented : Option<((i32, i32, i32), Vec<(i32, i32, i32)>)> = overlap_pair_scanner(oriented.get(p.0).unwrap(), input.get(p.1).unwrap());
        if next_oriented.is_some() {
            oriented.push(next_oriented.unwrap());
            input.remove(p.1);
        } else {
            break;
        }
    }

    let mut iters = 0;
    let mut last_i = 0;
    let mut last_j = 0;
    'outer: while input.len() > 0 && iters < 40 {
        for i in last_i .. oriented.len() {
            for j in last_j .. input.len() {
                let next_oriented : Option<((i32, i32, i32), Vec<(i32, i32, i32)>)> = overlap_pair_scanner(oriented.get(i).unwrap(), input.get(j).unwrap());
                if next_oriented.is_some() {
                    oriented.push(next_oriented.unwrap());
                    input.remove(j);
                    println!("found {}, {}", i, j);
                    last_j = j;
                    continue 'outer;
                }
            }
            last_i += 1;
            last_j = 0;
        }
        iters += 1;
    }

}

pub fn part_one_two() {
    let mut input : Vec<Vec<(i32, i32, i32)>> = include_str!("../inputs/day19.input")
        .split("\n\n")
        .map(|x| x.lines())
        .map(|x| x.skip(1)
            .map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap(), x.next().unwrap()))
            .collect()).collect();

    let timer = Timer::new();

    let mut oriented : Vec<((i32, i32, i32), Vec<(i32, i32, i32)>)> = Vec::new();

    oriented.push(((0, 0, 0), input.remove(0)));

    overlap_scanner(&mut oriented, &mut input);

    let mut all_points : HashSet<(i32, i32, i32)> = HashSet::new();

    for temp in &oriented {
        for point in &temp.1 {
            all_points.insert(*point);
        }
    }

    println!("day 19 part 1: {} in time {}ms", all_points.len(), timer.took().as_std().as_millis());

    let mut max = 0;
    for i in 0 .. oriented.len() {
        for j in i + 1 .. oriented.len() {
            let p1 = oriented.get(i).unwrap().0;
            let p2 = oriented.get(j).unwrap().0;
            let d = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2);
            if d > max {
                max = d;
            }
        }
    }
    
    println!("day 19 part 2: {} in time {}ms", max, timer.took().as_std().as_millis());
}
