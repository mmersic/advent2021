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

pub fn part_one_final() {
    let nums = include_str!("../inputs/day6.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut memo : HashMap<i32, usize> = HashMap::new();

    let result = nums.iter().map(|x| calc_opt_final(&mut memo, (79 - *x) as i32)).sum::<usize>();

    println!("day 6, part 1: {}", result);
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

fn calc_opt(memo: &mut HashMap<(usize, i32), usize>, n : usize, days : i32) -> usize {
    if memo.get(&(n, days)).is_some() {
        return *memo.get(&(n, days)).unwrap();
    } else {
        let mut rabbits = 1;
        if days < 0 {
            //do nothing
        } else if days == 0 {
            if n == 0 {
                rabbits += 1;
            }
        } else if n == 0 {
            rabbits = calc_opt(memo, 0, days - 7) + calc_opt(memo,0, days - 9);
        } else {
            rabbits = calc_opt(memo, 0, days - n as i32);
        }
        memo.insert((n, days), rabbits);
        return rabbits;
    }
}

fn calc_opt_final(memo: &mut HashMap<i32, usize>, days : i32) -> usize {
    if memo.get(&days).is_some() {
        return *memo.get(&days).unwrap();
    } else {
        let mut rabbits = 1;
        if days < 0 {
            //do nothing
        } else if days == 0 {
            rabbits += 1;
        } else if days > 0 {
            rabbits = calc_opt_final(memo,  days - 7) + calc_opt_final(memo, days - 9);
        }
        memo.insert(days, rabbits);
        return rabbits;
    }
}

pub fn part_two() {
    let nums = include_str!("../inputs/day6.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut memo : HashMap<(usize, u32), usize> = HashMap::new();

    let result = nums.iter().map(|x| calc(&mut memo, *x, 255)).sum::<usize>();

    println!("day 6, part 2: {}", result);
}

pub fn part_two_optimized() {
    let nums = include_str!("../inputs/day6.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut memo : HashMap<(usize, i32), usize> = HashMap::new();

    let result = nums.iter().map(|x| calc_opt(&mut memo, *x, 255)).sum::<usize>();
    
    println!("day 6, part 2: {}", result);
}

pub fn part_two_final() {
    let nums = include_str!("../inputs/day6.input").split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut memo : HashMap<i32, usize> = HashMap::new();

    let result = nums.iter().map(|x| calc_opt_final(&mut memo, (255 - *x) as i32)).sum::<usize>();

    println!("day 6, part 2: {}", result);
}