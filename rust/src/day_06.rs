use std::collections::HashMap;

pub fn part_one() {
    let mut nums = include_str!("../inputs/day6.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    
    for _ in 0 .. 80 {
        let mut new_fish = 0;
        for j in 0 .. nums.len() {
            if nums[j] == 0 {
                new_fish += 1;
                nums[j] = 6;
            } else {
                nums[j] -= 1;
            }
        }
        for _ in 0 .. new_fish {
            nums.push(8);
        }
    }
    
    println!("day 6, part 1: {}", nums.len());
}

fn calc(memo: &mut HashMap<(usize, u32), usize>, n : usize, days : u32) -> usize {
    if memo.get(&(n, days)).is_some() {
        return *memo.get(&(n, days)).unwrap();
    } else {
        let mut rabbits = 1;
        if days == 0 {
            if n == 0 {
                rabbits += 1;
            }            
        } else if n == 0 {
            rabbits = calc(memo, 6, days - 1) + calc(memo,8, days - 1);
        } else {
            rabbits = calc(memo, n - 1, days - 1);
        }
        memo.insert((n, days), rabbits);
        return rabbits;
    }
}

pub fn part_two() {
    let mut nums = include_str!("../inputs/day6.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut memo : HashMap<(usize, u32), usize> = HashMap::new();
    let mut result : Vec<usize> = Vec::new();
    
    for n in nums {
        result.push(calc(&mut memo, n, 255));
    }
    
    println!("day 6, part 2: {}", result.iter().sum::<usize>());
}