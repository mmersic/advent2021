
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

fn main() {
    day_one_part_one();
    day_one_part_two();
    day_two_part_one();
    day_two_part_two();
    day_three_part_one();
    day_three_part_two();
    day_four_part_one();
    day_four_part_two();
}
