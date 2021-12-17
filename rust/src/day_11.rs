use took::Timer;
use std::collections::{HashSet};

#[allow(dead_code)]
pub fn print_grid(a : &Vec<Vec<u8>>) {
    for i in 0 .. 10 {
        for j in 0 .. 10 {
            print!("{}", a[i][j])
        }
        println!();
    }
    println!();
}

pub fn part_one() {
    let timer = Timer::new();
    let mut a:Vec<Vec<u8>> = include_bytes!("../inputs/day11.input")
        .split(|&b| b == b'\n')
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<u8>>>();
    
    for y in 0 .. 10 {
        for x in 0 .. 10 {
            a[y][x] -= b'0';
        }
    }

    let neighbors: [(isize, isize); 8] = [(-1,0), (-1,-1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

    let mut flashes = 0;
    
    for _ in 0 .. 100 {
        for y in 0 .. 10 {
            for x in 0 .. 10 {
                a[y][x] += 1;
            }
        }

        let mut flashed_set : HashSet::<(usize,usize)> = HashSet::new();
        let mut something_flashed = true;
        while something_flashed {
            something_flashed = false;
            for y in 0..10 {
                for x in 0..10 {
                    if a[y][x] > 9 && !flashed_set.contains(&(x, y)) {
                        flashed_set.insert((x, y));
                        something_flashed = true;
                        neighbors.iter().for_each(|&(xx, yy)| {
                            if x as isize + xx < 10 && x as isize + xx >= 0 && y as isize + yy < 10 && y as isize + yy >= 0 {
                                a[(y as isize + yy) as usize][(x as isize + xx) as usize] += 1;
                            }
                        });
                    }
                }
            }
        }

        for y in 0 .. 10 {
            for x in 0 .. 10 {
                if a[y][x] > 9 {
                    flashes += 1;
                    a[y][x] = 0;
                }
            }
        }

    }
    
    println!("day 11, part 1: {} in time {}us", flashes, timer.took().as_std().as_micros());
}

pub fn part_two() {
    let timer = Timer::new();
    let mut a:Vec<Vec<u8>> = include_bytes!("../inputs/day11.input")
        .split(|&b| b == b'\n')
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<u8>>>();

    for y in 0 .. 10 {
        for x in 0 .. 10 {
            a[y][x] -= b'0';
        }
    }

    let neighbors: [(isize, isize); 8] = [(-1,0), (-1,-1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];
    
    let mut all_flashed = false;
    let mut iter_count = 0;
    
    while !all_flashed {
        iter_count += 1;
        for y in 0 .. 10 {
            for x in 0 .. 10 {
                a[y][x] += 1;
            }
        }

        let mut flashed_set : HashSet::<(usize,usize)> = HashSet::new();
        let mut something_flashed = true;
        while something_flashed {
            something_flashed = false;
            for y in 0..10 {
                for x in 0..10 {
                    if a[y][x] > 9 && !flashed_set.contains(&(x, y)) {
                        flashed_set.insert((x, y));
                        something_flashed = true;
                        neighbors.iter().for_each(|&(xx, yy)| {
                            if x as isize + xx < 10 && x as isize + xx >= 0 && y as isize + yy < 10 && y as isize + yy >= 0 {
                                a[(y as isize + yy) as usize][(x as isize + xx) as usize] += 1;
                            }
                        });
                    }
                }
            }
        }

        for y in 0 .. 10 {
            for x in 0 .. 10 {
                if a[y][x] > 9 {
                    a[y][x] = 0;
                }
            }
        }

        if flashed_set.len() == 100 {
            all_flashed = true;
        }
    }
    
    println!("day 11, part 2: {} in time {}us", iter_count, timer.took().as_std().as_micros());
}