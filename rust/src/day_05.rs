use std::fmt::{Display, Formatter};
use std::iter;
use nom::*;
use atoi::atoi;

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})->({}, {})", self.x1, self.y1, self.x2, self.y2)
    }
}

fn parse_day_five(input : &Vec<&str>) -> Vec<Line> {
    let mut lines : Vec<Line> = Vec::new();

    for i in input {
        let parts : Vec<&str> = i.split(" ").collect();
        let p1 : Vec<usize> = parts[0].split(",").map(|n| n.parse::<usize>().unwrap()).collect();
        let p2 : Vec<usize> = parts[2].split(",").map(|n| n.parse::<usize>().unwrap()).collect();
        if p1[0] == p2[0] {
            if p2[1] > p1[1] {
                lines.push(Line {
                    x1: p1[0],
                    y1: p1[1],
                    x2: p2[0],
                    y2: p2[1]
                });
            } else {
                lines.push(Line {
                    x1: p2[0],
                    y1: p2[1],
                    x2: p1[0],
                    y2: p1[1]
                });
            }
        } else {
            if p2[0] > p1[0] {
                lines.push(Line {
                    x1: p1[0],
                    y1: p1[1],
                    x2: p2[0],
                    y2: p2[1]
                });
            } else {
                lines.push(Line {
                    x1: p2[0],
                    y1: p2[1],
                    x2: p1[0],
                    y2: p1[1]
                });
            }
        }
    }

    return lines;
}

// fn print_grid(grid : &Vec<u8>, x : usize, y : usize) {
//     for i in 0 .. y {
//         for j in 0 .. x {
//             print!("{}",grid[i*y+j]);
//         }
//         println!();
//     }
// } 



pub fn part_one() {
    let input = include_str!("../inputs/day5.input").lines()
        .collect::<Vec<&str>>();
    let lines : Vec<Line> = parse_day_five(&input);

    let mut grid : Vec<Vec<usize>> = Vec::new();

    for i in 0 .. 1000 {
        grid.push(Vec::new());
        for _ in 0 .. 1000 {
            grid[i].push(0);
        }
    }

    for line in lines {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            for i in line.y1..=line.y2 {
                for j in line.x1..=line.x2 {
                    grid[i][j] += 1;
                }
            }
        } else {

        }
    }

    let mut count : usize = 0;
    for i in 0 .. 1000 {
        for j in 0 .. 1000 {
            if grid[i][j] > 1 {
                count += 1;
            }
        }
    }

    println!("day 5, part 1: {}", count);
}

pub fn part_two() {
    let input = include_str!("../inputs/day5.input").lines()
        .collect::<Vec<&str>>();
    let lines : Vec<Line> = parse_day_five(&input);

    let mut grid : Vec<Vec<usize>> = Vec::new();

    for i in 0 .. 1000 {
        grid.push(Vec::new());
        for _ in 0 .. 1000 {
            grid[i].push(0);
        }
    }

    for line in lines {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            for i in line.y1..=line.y2 {
                for j in line.x1..=line.x2 {
                    grid[i][j] += 1;
                }
            }
        } else {
            if line.y1 > line.y2 {
                let mut x = line.x1;
                let mut y = line.y1;
                while x != line.x2 && y != line.y2 {
                    grid[y][x] += 1;
                    x += 1;
                    y -= 1;
                }
                grid[y][x] += 1;
            } else {
                let mut x = line.x1;
                let mut y = line.y1;
                while x != line.x2 && y != line.y2 {
                    grid[y][x] += 1;
                    x += 1;
                    y += 1;
                }
                grid[y][x] += 1;
            }
        }
    }

    let mut count : usize = 0;
    for i in 0 .. 1000 {
        for j in 0 .. 1000 {
            if grid[i][j] > 1 {
                count += 1;
            }
        }
    }

    println!("day 5, part 2: {}", count);
}

//this soln from:
//https://github.com/timvisee/advent-of-code-2021/blob/master/day05a/src/main.rs
pub fn part_one_optimized() {
    let map = include_bytes!("../inputs/day5.input")
        .split(|b| *b == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .filter(|(x, y, xx, yy)| x == xx || y == yy)
        .fold(vec![0u8; 1000 * 1000], |mut map, (x, y, xx, yy)| {
            if x == xx {
                (y..=yy).for_each(|y| map[(x + y * 1000) as usize] += 1);
            } else {
                (x..=xx).for_each(|x| map[(x + y * 1000) as usize] += 1);
            }
            map
        })
        .into_iter()
        .filter(|c| *c >=2)
        .count();

    println!("day 5, part 1: {}", map);
}

//my butchering of part one optimized:
pub fn part_two_optimized_one() {
    let map = include_bytes!("../inputs/day5.input")
        .split(|b| *b == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            if (x == xx && y > yy) || (y == yy && x > xx) || (x != xx && y != yy && x > xx) {
                (xx, yy, x, y)
            }  else {
                (x, y, xx, yy)
            }
        })
        .fold(vec![0u8; 1000 * 1000], |mut map, (x, y, xx, yy)| {
            if x == xx {
                (y..=yy).for_each(|y| map[(x + y * 1000) as usize] += 1);
            } else if y == yy {
                (x..=xx).for_each(|x| map[(x + y * 1000) as usize] += 1);
            } else {
                let mut x1 = x;
                let mut y1 = y;
                if y > yy {
                    (yy..=y).for_each(|_| { map[(x1 + y1 * 1000) as usize] += 1; x1 += 1; y1 -= 1; });
                } else {
                    (y..=yy).for_each(|_| { map[(x1 + y1 * 1000) as usize] += 1; x1 += 1; y1 += 1; });
                }
            }
            map
        });

    let count = map.into_iter()
        .filter(|c| *c >=2)
        .count();

    println!("day 5, part 2: {}", count);
}

//This soln from: 
// https://github.com/timvisee/advent-of-code-2021/blob/master/day05b/src/main.rs
pub fn part_two_optimized_two() {
    let map = include_bytes!("../inputs/day5.input")
        .split(|b| *b == b'\n')
        .fold(
            vec![0u8; 1000 * 1000],
            |mut map, entry| {
                let ((x, y), (xx, yy)) = line(entry).unwrap().1;
                let range =
                    |a: isize, b: isize| iter::successors(Some(a), move |n| Some(n + (b - a).signum()));
                range(x, xx)
                    .cycle()
                    .zip(range(y, yy).cycle())
                    .take((x - xx).abs().max((y - yy).abs()) as usize + 1)
                    .for_each(|(x, y)| map[(x + y * 1000) as usize] += 1);
                map
            },
        );

    println!("day 5, part 2: {}", map.into_iter().filter(|c| *c >= 2).count());
}

named!(isize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (isize, isize)>, separated_pair!(isize, char!(','), isize));
named!(line<&[u8], ((isize, isize), (isize, isize))>, separated_pair!(coord, tag!(" -> "), coord));