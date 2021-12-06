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



//This is from:
//https://github.com/timvisee/advent-of-code-2021/blob/master/day04a/src/main.rs
//
//How marking is done:
//Order of operations here: *m |= 1 << *i)
//First shift 1 to the left i places, then or the result of the shift with m.  This "marks" the ith position on the board.
//
//What is this: (mark >> i & 1 ^ 1) * n as u32 * num as u32)
//Basically, a compact if statement.  If the ith position is marked, set to 1,
//then the result of the xor will be 0 and the result of the expression is 0,
//otherwise xor will be 1 and the result of the expression is n * num, which is subsequently
//summed for the answer.
//Shift mark to the right i times to but the ith bit in position 1.
//And the result with 1, so if the ith position was marked the result here is 1, 0 otherwise
//xor the result with 1, so if the ith position was marked, the result here is 0, 1 otherwise
//multiply the result (either 0 or 1) times n (the number on the board)
//multiply the result times num (the winning number).
use std::collections::BTreeMap;
pub fn part_one_final() {
    const ROW: u32 = 0b11111;
    const COL: u32 = 0b100001000010000100001;

    let (nums, boards) = include_str!("../inputs/day4.input").split_once("\n\n").unwrap();

    //vector of pair of (map of number on board to position of number on board, and u32 that starts at 0 - this is marks on the board)
    let mut boards: Vec<(BTreeMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .find_map(|n| {
            boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)//i is the position of n on this board. m is the marks. All the marks fit in u32 since there are on 25 of them at most.
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))//check each column and each row for a bingo by shifting the marks and comparing with the COL and ROW winning patterns.
                    .map(|_| (b.clone(), *m, n)) //tuple of (clone the winning board, it's marks, and the winning number).
            })
        })
        .unwrap();

    println!(
        "day 4, part 1: {}",
        board
            .into_iter()
            .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * num as u32)
            .sum::<u32>()
    );
}