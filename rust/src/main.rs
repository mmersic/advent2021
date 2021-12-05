use std::fmt::{Display, Formatter};
use std::iter;
use nom::*;
use atoi::atoi;

fn day_one_part_one() {
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

fn day_one_part_two() {
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

fn day_two_part_one() {
    let lines = include_str!("../inputs/day2.input")
        .lines();
    
    let mut h: i32 = 0;
    let mut d : i32 = 0;
    
    for line in lines {
        let v: Vec<&str> = line.split(" ").collect();
        if v[0] == "forward" {
            h += v[1].parse::<i32>().unwrap();
        } else if v[0] == "up" {
            d -= v[1].parse::<i32>().unwrap();
        } else if v[0] == "down" {
            d += v[1].parse::<i32>().unwrap();
        }
    }
    
    println!("day 2, part 1: {}", h*d);
    
}

fn day_two_part_two() {
    let lines = include_str!("../inputs/day2.input")
        .lines();

    let mut h: i32 = 0;
    let mut a: i32 = 0;
    let mut d : i32 = 0;

    for line in lines {
        let v: Vec<&str> = line.split(" ").collect();
        if v[0] == "forward" {
            h += v[1].parse::<i32>().unwrap();
            d += v[1].parse::<i32>().unwrap() * a;
        } else if v[0] == "up" {
            a -= v[1].parse::<i32>().unwrap();
        } else if v[0] == "down" {
            a += v[1].parse::<i32>().unwrap();
        }
    }

    println!("day 2, part 1: {}", h*d);

}

fn day_three_part_one() {
    let lines = include_str!("../inputs/day3.input")
        .lines().collect::<Vec<&str>>();

    let mut e: i32 = 0;
    let mut g: i32 = 0;

    let mut v: Vec<i32> = Vec::new();
    
    for _ in 0..lines[0].len() {
        v.push(0);
    }
    
    let one = "1".as_bytes()[0];
    
    for line in &lines {
        let b = line.as_bytes();
        for i in 0 .. b.len() {
            if b[i] == one {
                v[i] += 1;
            }
        }
    }

    for n in v {
        e *= 2;
        g *= 2;
        if n > 500 {
            g+=1;
        } else {
            e+=1;
        }
    }
    
    println!("day 3, part 1: {}", e*g);
}

fn bit_count(lines: &Vec<&str>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    
    for _ in 0..lines[0].len() {
        v.push(0);
    }

    let one = "1".as_bytes()[0];

    for line in &*lines {
        let b = line.as_bytes();
        for i in 0 .. b.len() {
            if b[i] == one {
                v[i] += 1;
            }
        }
    }

    return v;
}

fn str_to_int(s : &str) -> i32 {
    let mut x : i32 = 0;

    let one = "1".as_bytes()[0];

    
    for n in s.as_bytes() {
        x *= 2;
        if *n == one {
            x+=1;
        } 
    }
    
    return x;
}

fn day_three_part_two() {
    let lines = include_str!("../inputs/day3.input")
        .lines().collect::<Vec<&str>>();

    let one = "1".as_bytes()[0];
    let zero = "0".as_bytes()[0];

    let mut result : Vec<&str> = lines.to_vec();

    for i in 0 .. lines[0].len() {
        let v: Vec<i32> = bit_count(&result);
        let mut rl_half: usize = result.len()/2;
        if result.len() % 2 == 1 {
            rl_half += 1;
        }
        
        result = result.iter().filter(|x| (v[i] >= rl_half as i32 && x.as_bytes()[i] == one) || (v[i] < rl_half as i32 && x.as_bytes()[i] == zero)).map(|x| *x).collect::<Vec<&str>>();
        if result.len() == 1 {
            break;
        }
    }
    
    let o2 : i32 = str_to_int(&result[0]);

    let mut result : Vec<&str> = lines.to_vec();

    for i in 0 .. lines[0].len() {
        let v: Vec<i32> = bit_count(&result);
        let mut rl_half: usize = result.len()/2;
        if result.len() % 2 == 1 {
            rl_half += 1;
        }

        result = result.iter().filter(|x| (v[i] < rl_half as i32 && x.as_bytes()[i] == one) || (v[i] >= rl_half as i32 && x.as_bytes()[i] == zero)).map(|x| *x).collect::<Vec<&str>>();
        if result.len() == 1 {
            break;
        }
    }

    let co2 : i32 = str_to_int(&result[0]);

    println!("day 3, part 2: {}", o2*co2);
}

fn parse_day_four(lines : &Vec<&str>) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let called_nums : Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut bingo_boards : Vec<Vec<Vec<i32>>> = Vec::new();
    for i in 1 .. lines.len() {
        let bingo_s = lines[i];
        let mut bingo_board : Vec<Vec<i32>> = Vec::new();
        for b in bingo_s.lines() {
            bingo_board.push(b.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect());
        }
        bingo_boards.push(bingo_board);
    }
    
    return (called_nums, bingo_boards);
}

// fn print_bingo_board(b : &Vec<Vec<i32>>) {
//     for l in b {
//         for n in l {
//             print!("{} ", n);
//         }
//         println!();
//     }
// }

fn mark(n : i32, boards: &mut Vec<Vec<Vec<i32>>>) {
    for i in 0 .. boards.len() {
        for j in 0 .. boards[0].len() {
            for k in 0..boards[0][0].len() {
                if boards[i][j][k] == n {
                    boards[i][j][k] = -1;
                }
            }
        }
    }
}

fn sum_board(board: &Vec<Vec<i32>>) -> i32 {
    let mut bingo = false;
    
    'next_a: for i in 0 .. board.len() {
        for j in 0 .. board[0].len() {
           if board[i][j] != -1 {
               continue 'next_a;
           }
        }
        bingo = true;
    }
    
    if !bingo {
        'next_b: for i in 0 .. board.len() {
            for j in 0 .. board[0].len() {
                if board[j][i] != -1 {
                    continue 'next_b;
                }
            }
            bingo = true;
        }
    }
    
    if bingo {
        let mut sum = 0;
        for i in 0 .. board.len() {
            for j in 0 .. board[0].len() {
                if board[i][j] != -1 {
                    sum += board[i][j];
                }
            }
        }
        return sum;
    }
    
    return -1;
}

fn sum_boards(boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    for i in 0 .. boards.len() {
        let score = sum_board(&boards[i]);
        if score != -1 {
            return score;
        }
    }
    
    return -1;
}

fn day_four_part_one() {
    let lines = include_str!("../inputs/day4.input").split("\n\n")
        .collect::<Vec<&str>>();
    
    let (called_nums, mut bingo_boards) = parse_day_four(&lines);

    for n in &called_nums {
        mark(*n, &mut bingo_boards);
        let sum = sum_boards(&bingo_boards);
        if sum > -1 {
            println!("day 4, part 1: {}", sum*n);
            break;
        }
    }
}

fn winning_boards(boards: &Vec<Vec<Vec<i32>>>) -> usize {
    let mut count = 0;
    
    for i in 0 .. boards.len() {
        if sum_board(&boards[i]) > -1 {
            count += 1;
        }
    }
    
    return count;
}

fn last_board(boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    for i in 0 .. boards.len() {
        if sum_board(&boards[i]) == -1 {
            return i as i32;
        }
    }
    return -1;
}

fn day_four_part_two() {
    let lines = include_str!("../inputs/day4.input").split("\n\n")
        .collect::<Vec<&str>>();

    let (called_nums, mut bingo_boards) = parse_day_four(&lines);

    let board_count = bingo_boards.len();
    let mut last_board_idx = -1; 
    
    for n in &called_nums {
        mark(*n, &mut bingo_boards);
        let winning_boards = winning_boards(&bingo_boards);
        if last_board_idx == -1 && winning_boards == board_count-1 {
            last_board_idx = last_board(&bingo_boards);
        }
        if winning_boards == board_count {
            println!("day 4, part 2: {}", sum_board(&bingo_boards[last_board_idx as usize])*n);
            break;
        }
    }
}

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

fn print_grid(grid : &Vec<u8>, x : usize, y : usize) {
    for i in 0 .. y {
        for j in 0 .. x {
            print!("{}",grid[i*y+j]);
        }
        println!();
    }
} 



fn day_five_part_one() {
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

fn day_five_part_two() {
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
fn day_five_part_one_optimized() {
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
fn day_five_part_two_optimized_one() {
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
                    (yy..=y).for_each(|y| { map[(x1 + y1 * 1000) as usize] += 1; x1 += 1; y1 -= 1; });
                } else {
                    (y..=yy).for_each(|y| { map[(x1 + y1 * 1000) as usize] += 1; x1 += 1; y1 += 1; });    
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
fn day_five_part_two_optimized_two() {
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

fn main() {
    day_one_part_one();
    day_one_part_two();
    day_two_part_one();
    day_two_part_two();
    day_three_part_one();
    day_three_part_two();
    day_four_part_one();
    day_four_part_two();
    day_five_part_one();
    day_five_part_two();
    day_five_part_one_optimized();
    day_five_part_two_optimized_one();
    day_five_part_two_optimized_two();
    
}

named!(isize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (isize, isize)>, separated_pair!(isize, char!(','), isize));
named!(line<&[u8], ((isize, isize), (isize, isize))>, separated_pair!(coord, tag!(" -> "), coord));