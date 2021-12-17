use took::Timer;
use std::collections::{HashSet};


pub fn part_one() {
    let timer = Timer::new();

    let (points_str, folds_str) = include_str!("../inputs/day13.input")
        .split_once("\n\n").unwrap();
    
    let mut points = points_str
        .lines()
        .map(|p| p.split_once(",").unwrap())
        .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()) ).collect::<HashSet<(usize, usize)>>();
    
    let folds = folds_str
        .lines()
        .map(|l| l.split_once("fold along ").unwrap().1)
        .map(|f| f.split_once("=").unwrap())
        .map(|p| (p.0, p.1.parse().unwrap()))
        .collect::<Vec<(&str, usize)>>();
    
    if folds[0].0 == "y" {
        points = fold_y(points, folds[0].1);
    } else {
        points = fold_x(points, folds[0].1);
    }
    
    println!("day 13, part 1: {} in time {}us", points.len(), timer.took().as_std().as_micros());
}

fn fold_y(points : HashSet<(usize, usize)>, y : usize) -> HashSet<(usize, usize)> {
    return points.iter().fold(<HashSet<(usize, usize)>>::new(), |mut acc, p| {
        if p.1 < y {
            acc.insert(*p);
        } else {
            acc.insert((p.0, y - (p.1 - y)));
        }
        acc
    });
}

fn fold_x(points : HashSet<(usize, usize)>, x : usize) -> HashSet<(usize, usize)> {
    return points.iter().fold(<HashSet<(usize, usize)>>::new(), |mut acc, p| {
        if p.0 < x {
            acc.insert(*p);
        } else {
            acc.insert((x-(p.0-x), p.1));
        }
        acc
    });
}


pub fn part_two() {
    let timer = Timer::new();

    let (points_str, folds_str) = include_str!("../inputs/day13.input")
        .split_once("\n\n").unwrap();

    let mut points = points_str
        .lines()
        .map(|p| p.split_once(",").unwrap())
        .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()) ).collect::<HashSet<(usize, usize)>>();

    let folds = folds_str
        .lines()
        .map(|l| l.split_once("fold along ").unwrap().1)
        .map(|f| f.split_once("=").unwrap())
        .map(|p| (p.0, p.1.parse().unwrap()))
        .collect::<Vec<(&str, usize)>>();

    println!("points len: {}", points.len());

    for fold in folds {
        if fold.0 == "y" {
            points = fold_y(points, fold.1);
        } else {
            points = fold_x(points, fold.1);
        }
    }

    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    for y in 0 ..=max_y {
        for x in 0 ..=max_x {
            if points.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("day 13, part 2, see above, in time {}us", timer.took().as_std().as_micros());

}


fn uint(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

pub fn part_two_optimized() {
    let timer = Timer::new();

    let (points_str, folds_str) = include_str!("../inputs/day13.input")
        .split_once("\n\n").unwrap();

    let mut points: Vec<_> = points_str
        .lines()
        .map(|p| p.split_once(",").unwrap())
        .map(|(x,y ) | (uint(x), uint(y)) ).collect();

    let folds: Vec<_> = folds_str
        .lines()
        .map(|l| l.split_once("=").unwrap())
        .map(|p| (p.0.ends_with("x"), uint(p.1)))
        .collect();

    for fold in folds {
        for (x,y) in &mut points {
            let coord = if fold.0 { x } else { y };
            *coord = fold.1-coord.abs_diff(fold.1);
        }
    }
    
    let (max_x, max_y) = points.iter()
        .fold((0,0), |(max_x, max_y), (x, y) | (max_x.max(*x), max_y.max(*y)));

    let point_set : HashSet<_> = points.iter().collect();

    for y in 0 ..=max_y {
        for x in 0 ..=max_x {
            print!("{}", if point_set.contains(&(x,y)) { "░" } else { " " });
        }
        println!();
    }
    
    println!("day 13, part 2, see above, in time {}us", timer.took().as_std().as_micros());
}