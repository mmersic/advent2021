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

pub fn part_one() {
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

pub fn part_two() {
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