pub fn part_one() {
    let nums = include_str!("../inputs/day1.input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut inc_count = 0;

    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            inc_count += 1;
        }
    }

    println!("day 1, part 1: {}", inc_count);
}

pub fn part_one_optimized() {
    let nums = include_str!("../inputs/day1.input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let count = (1..nums.len())
        .filter(|x| nums[*x] > nums[*x-1])
        .count();

    println!("day 1, part 1: {}", count);
}

//Mostly from:
//https://github.com/timvisee/advent-of-code-2021/blob/master/day01a/src/main.rs
pub fn part_one_final() {
    let count = include_str!("../inputs/day1.input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
        .array_windows()
        .filter(|[x, y]| x < y)
        .count();

    println!("day 1, part 1: {}", count);
}

pub fn part_two() {
    let nums = include_str!("../inputs/day1.input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut inc_count = 0;

    for x in 3..nums.len() {
        if nums[x] > nums[x-3] {
            inc_count += 1;
        }
    }

    println!("day 1, part 2: {}", inc_count);
}

pub fn part_two_optimized() {
    let nums = include_str!("../inputs/day1.input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();


    let count = (3..nums.len())
        .filter(|x| nums[*x] > nums[*x-3])
        .count();
    
    println!("day 1, part 2: {}", count);
}

pub fn part_two_final() {
    let count = include_str!("../inputs/day1.input")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>()
        .array_windows()
        .filter(|[x, _, _, y]| x < y)
        .count();

    println!("day 1, part 2: {}", count);
}