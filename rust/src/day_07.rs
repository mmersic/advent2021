use std::collections::HashMap;

pub fn part_one() {
    let nums = include_str!("../inputs/day7.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let max = nums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let mut memo : HashMap<i32, i32> = HashMap::new();
    
    
    (0..*max).for_each(|x| {
       memo.insert(x, nums.iter().map(|y| {
           (x-*y).abs() 
       }).sum());
    });

    let min = memo.iter().min_by(|a, b| {
        a.1.cmp(b.1)
    }).unwrap();
    
    println!("day 7, part 1: {}", min.1);
    
}

pub fn part_two() {
    let nums = include_str!("../inputs/day7.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let max = nums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let mut memo : HashMap<i32, i32> = HashMap::new();


    (0..*max).for_each(|x| {
        memo.insert(x, nums.iter().map(|y| {
            let a = (x-*y).abs();
            (a*(a+1))/2
        }).sum());
    });

    let min = memo.iter().min_by(|a, b| {
        a.1.cmp(b.1)
    }).unwrap();

    println!("day 7, part 2: {}", min.1);

}

pub fn part_one_optimized() {
    let mut nums = include_str!("../inputs/day7.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    nums.sort();
    
    let mid = nums.len()/2;
    let med = nums.get(mid).unwrap();
    let result : i32 = nums.iter().map(|x| (x-med).abs()).sum();
    
    println!("day 7, part 1: {}", result);
}

//this soln from:
//https://github.com/timvisee/advent-of-code-2021/blob/master/day07b/src/main.rs
pub fn part_two_optimized() {
    let mut nums = include_str!("../inputs/day7.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let mean = (nums.iter().sum::<i32>() / nums.len() as i32..).take(2);
    
    let result : i32 = mean.map(|x| {
        nums.iter().map(|y| {
            let a = (y - x).abs();
            (a * (a + 1)) / 2
        }).sum()
    }).min().unwrap();
    
    println!("day 7, part 2: {}", result);
}