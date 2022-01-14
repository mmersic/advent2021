use std::collections::{HashMap, HashSet};

pub fn part_one() {
    let mut p1 = 9;
    let mut p2 = 0;
    
    let mut s1 = 0;
    let mut s2 = 0;
    
    let mut die = 1;
    let mut roll_count = 0;
    
    loop {
        let mut roll = 0;
        if die < 98 {
            roll = die * 3 + 3;
            println!("p1: {}, {}, {}: {}", die, die+1, die+2, roll);
            die += 3;
        } else if die == 98 {
            roll = die * 3 + 3;
            println!("p1: {}, {}, {}: {}", die, die+1, die+2, roll);
            die = 1;
        } else if die == 99 {
            roll = 99 + 100 + 1;
            println!("p1: {}, {}, {}: {}", 99, 100, 1, roll);
            die = 2;
        } else if die == 100 {
            roll = 100 + 1 + 2;
            println!("p1: {}, {}, {}: {}", 100, 1, 2, roll);
            die = 3;
        }

        p1 = ((p1 + roll) % 10);
        s1 += if p1 == 0 { 10 } else { p1 };
        
        roll_count += 3;
        if s1 >= 1000 { break; }

        if die < 98 {
            roll = die * 3 + 3;
            println!("p2: {}, {}, {}: {}", die, die+1, die+2, roll);
            die += 3;
        } else if die == 98 {
            roll = die * 3 + 3;
            println!("p2: {}, {}, {}: {}", die, die+1, die+2, roll);
            die = 1;
        } else if die == 99 {
            roll = 99 + 100 + 1;
            println!("p2: {}, {}, {}: {}", 99, 100, 1, roll);
            die = 2;
        } else if die == 100 {
            roll = 100 + 1 + 2;
            println!("p2: {}, {}, {}: {}", 100, 1, 2, roll);
            die = 3;
        }

        p2 = ((p2 + roll) % 10);
        s2 += if p2 == 0 { 10 } else { p2 };
        roll_count += 3;
        
        if s2 >= 1000 { break; }
    }
    
    println!("day 21, part 1: {}", if s1 < 1000 { roll_count*s1 } else { roll_count*s2 });
    
}

fn play(turn : usize, r1 : usize, r2 : usize, r3 : usize, mut p1: usize, mut p2: usize, mut s1: usize, mut s2: usize, map: &mut HashMap::<String, (usize, usize)>) -> (usize, usize) {
    let mut p1_wins = 0 as usize;
    let mut p2_wins = 0 as usize;

    let key = format!("{}.{}.{}.{}.{}.{}.{}.{}", turn, r1, r2, r3, p1, p2, s1, s2);

    if map.contains_key(&*key) {
        return *map.get(&key).unwrap();
    }

    if turn == 1 {
        p1 = ((p1 + r1 + r2 + r3) % 10);
        s1 += if p1 == 0 { 10 } else { p1 };
        if s1 >= 21 {
            map.insert(key, (1, 0));
            return (1, 0);
        }
        for i in 1 .. 4 {
            for j in 1 .. 4 {
                for k in 1 .. 4 {
                    let (x1, x2) = play(2, i, j, k, p1, p2, s1, s2, map);
                    p1_wins += x1;
                    p2_wins += x2;
                }
            }
        }
    } else if turn == 2 {
        p2 = ((p2 + r1 + r2 + r3) % 10);
        s2 += if p2 == 0 { 10 } else { p2 };
        if s2 >= 21 {
            map.insert(key, (0, 1));
            return (0, 1);
        }
        for i in 1 .. 4 {
            for j in 1 .. 4 {
                for k in 1 .. 4 {
                    let (x1, x2) = play(1, i, j, k, p1, p2, s1, s2, map);
                    p1_wins += x1;
                    p2_wins += x2;
                }
            }
        }
    }

    map.insert(key, (p1_wins, p2_wins));

    return (p1_wins, p2_wins);
}

fn play_dirac(turn: usize, p1 : usize, p2 : usize, s1 : usize, s2 : usize, mut map: HashMap::<String, (usize, usize)>) -> (usize, usize) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for i in 1 .. 4 {
        for j in 1 .. 4 {
            for k in 1 .. 4 {
                let (x1, x2) = play(1, i, j, k, p1, p2, s1, s2, &mut map);
                p1_wins += x1;
                p2_wins += x2;
            }
        }
    }

    return (p1_wins, p2_wins);
}

pub fn part_two() {
    let map : HashMap<String, (usize, usize)> = HashMap::new(); 
    
    let (p1_wins, p2_wins) = play_dirac(1, 9, 10, 0, 0, map);

    //sample: 444356092776315
    println!("day 21, part 2: {}", p1_wins.max(p2_wins));

}