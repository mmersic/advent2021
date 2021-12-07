use std::collections::HashMap;

pub fn part_one() {
    let nums : Vec<usize> = include_str!("../inputs/day7.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let max = nums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let mut memo : HashMap<usize, usize> = HashMap::new();
    
    
    (0..*max).for_each(|x| {
       memo.insert(x, nums.iter().map(|y| {
           x.abs_diff(*y) 
       }).sum());
    });

    let min = memo.iter().min_by(|a, b| {
        a.1.cmp(b.1)
    }).unwrap();
    
    println!("day 7, part 1: {}", min.1);
    
}

pub fn part_two() {
    let nums : Vec<usize> = include_str!("../inputs/day7.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let max = nums.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let mut memo : HashMap<usize, usize> = HashMap::new();


    (0..*max).for_each(|x| {
        memo.insert(x, nums.iter().map(|y| {
            let a = x.abs_diff(*y);
            (a*(a+1))/2
        }).sum());
    });

    let min = memo.iter().min_by(|a, b| {
        a.1.cmp(b.1)
    }).unwrap();

    println!("day 7, part 2: {}", min.1);

}