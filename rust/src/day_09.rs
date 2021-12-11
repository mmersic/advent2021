use itertools::Itertools;
use took::Timer;


pub fn part_one() {
    let timer = Timer::new();
    let nums = include_str!("../inputs/day9.input")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    for i in 0 .. nums.len() {
        for j in 0 .. nums[0].len() as usize {
            if j+1 < nums[0].len() {
                if nums[i][j] >= nums[i][j+1] {
                    continue;
                } 
            } 
            if j > 0 {
                if nums[i][j] >= nums[i][j-1] {
                    continue;
                }
            }                
            if i+1 < nums.len() {
                if nums[i][j] >= nums[i+1][j] {
                    continue;
                }
            }
            if i > 0 {
                if nums[i][j] >= nums[i-1][j] {
                    continue;
                }
            }
            sum += nums[i][j].to_digit(10).unwrap()+ 1;
        }
    }
    
    println!("day 9, part 1: {} in time {}us", sum, timer.took().as_std().as_micros());
}

pub fn part_one_optimized() {
    let timer = Timer::new();
    let nums = include_bytes!("../inputs/day9.input")
        .split(|b| *b == b'\n')
        .flatten()
        .map(|b| *b - b'0')
        .collect::<Vec<u8>>();
    
    let mut sum : usize = 0;
    
    for i in 0 .. 100 {
        for j in 0 .. 100 {
            if    (j < 99 && nums[i*100 + j] >= nums[i*100 + j + 1])
                || (j > 0 && nums[i*100 + j] >= nums[i*100+j-1])
                || (i < 99 && nums[i*100 + j] >= nums[(i+1)*100 + j])
                || (i > 0  && nums[i*100 + j] >= nums[(i-1)*100 + j]) {
                continue;
            }
            sum += (nums[i*100 + j]+1) as usize;
        }
    }
    
    println!("day 9, part 1: {} in time {}us", sum, timer.took().as_std().as_micros());
}

fn basin(nums : &Vec<Vec<char>>, visited : &mut Vec<Vec<bool>>, i : usize, j : usize) -> usize {
    if i >= (*nums).len() || j >= nums[0].len() || visited[i][j] {
        return 0;
    } else {
        visited[i][j] = true;
        if nums[i][j] == '9' {
            return 0;
        } else {
            let mut sum : usize = 0;
            if j > 0 {
                sum += basin(nums, visited,i, j-1)
            }
            if i > 0 {
                sum += basin(nums, visited,i-1, j)
            }
            return sum + 1 + basin(nums, visited, i, j+1)  + basin(nums, visited,i+1, j);
        }
    }
}

fn basin_size(nums : &Vec<Vec<char>>, visited : &mut Vec<Vec<bool>>, i : usize, j : usize) -> usize {
    let mut sum : usize = 0;
    if j > 0 {
        sum += basin(nums, visited, i, j-1);
    }
    if i > 0 {
        sum += basin(nums, visited,i-1, j);
    }
    return sum + basin(nums, visited, i, j+1) + basin(nums, visited,i+1, j);
}

pub fn part_two() {
    let timer = Timer::new();
    let nums = include_str!("../inputs/day9.input")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut visited: Vec<Vec<bool>> = Vec::new();

    for _ in 0..nums.len() {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..nums[0].len() {
            row.push(false);
        }
        visited.push(row);
    }

    let mut basin_sizes: Vec<usize> = Vec::new();

    for i in 0..nums.len() {
        for j in 0..nums[0].len() as usize {
            if j + 1 < nums[0].len() {
                if nums[i][j] >= nums[i][j + 1] {
                    continue;
                }
            }
            if j > 0 {
                if nums[i][j] >= nums[i][j - 1] {
                    continue;
                }
            }
            if i + 1 < nums.len() {
                if nums[i][j] >= nums[i + 1][j] {
                    continue;
                }
            }
            if i > 0 {
                if nums[i][j] >= nums[i - 1][j] {
                    continue;
                }
            }
            basin_sizes.push(basin_size(&nums, &mut visited, i, j));
        }
    }

    basin_sizes = basin_sizes.iter().sorted().rev().map(|x| *x).collect::<Vec<usize>>();
    
    println!("day 9, part 2: {} in time {}us", (basin_sizes[0] * basin_sizes[1] * basin_sizes[2]), timer.took().as_std().as_micros());
}

fn basin_optimized(nums : &mut Vec<u8>, i : usize, j : usize) -> usize {
    if i >= 100 || j >= 100 || nums[i*100 + j] == 10 {
        return 0;
    } else {
        if nums[i*100+j] == 9 {
            return 0;
        } else {
            nums[i*100+j] = 10;
            let mut sum : usize = 0;
            if j > 0 {
                sum += basin_optimized(nums, i, j-1)
            }
            if i > 0 {
                sum += basin_optimized(nums, i-1, j)
            }
            return sum + 1 + basin_optimized(nums, i, j+1)  + basin_optimized(nums,i+1, j);
        }
    }

}

fn basin_size_optimized(nums : &mut Vec<u8>, i : usize, j : usize) -> usize {
    let mut sum : usize = 0;
    if j > 0 {
        sum += basin_optimized(nums, i, j-1);
    }
    if i > 0 {
        sum += basin_optimized(nums, i-1, j);
    }
    return sum + basin_optimized(nums, i, j+1) + basin_optimized(nums, i+1, j);
}

pub fn part_two_optimized() {
    let timer = Timer::new();
    let mut nums = include_bytes!("../inputs/day9.input")
        .split(|b| *b == b'\n')
        .flatten()
        .map(|b| *b - b'0')
        .collect::<Vec<u8>>();

    let mut sizes : Vec<usize> = Vec::new();

    for i in 0 .. 100 {
        for j in 0 .. 100 {
            if (j < 99 && nums[i*100 + j] >= nums[i*100 + j + 1])
                || (j > 0 && nums[i*100 + j] >= nums[i*100+j-1])
                || (i < 99 && nums[i*100 + j] >= nums[(i+1)*100 + j])
                || (i > 0  && nums[i*100 + j] >= nums[(i-1)*100 + j]) {
                continue;
            }
            sizes.push(basin_size_optimized(&mut nums, i, j));
        }
    }
    
    sizes = sizes.iter().sorted().rev().map(|x| *x).collect::<Vec<usize>>();
    
    println!("day 9, part 2: {} in time {}us", (sizes[0] * sizes[1] * sizes[2]), timer.took().as_std().as_micros());
}