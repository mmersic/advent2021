use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_ints(path: &str) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let lines = read_lines(path).unwrap();
    
    for line in lines {
        let num = line.unwrap().parse::<i32>().unwrap();
        out.push(num);
    }
    
    return out;
}

fn day_one_part_one() {
    let nums = read_ints("inputs/day1.input");

    let mut inc_count = 0;

    for i in 1..nums.len() {
        if nums[i] > nums[i-1] {
            inc_count += 1;
        }
    }
    
    println!("part1: {}", inc_count);
    
}

fn day_one_part_two() {
    let nums = read_ints("inputs/day1.input");
    
    let mut inc_count = 0;

    for x in 3..nums.len() {
        let w1 = nums[x-3]+nums[x-2]+nums[x-1];
        let w2 = nums[x-2]+nums[x-1]+nums[x];
        
        if w2 > w1 {
            inc_count += 1;
        }
    }

    println!("part2: {}", inc_count);
}

fn main() {
    day_one_part_one();
    day_one_part_two();
}
