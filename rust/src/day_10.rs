use took::Timer;
use std::collections::{HashSet, HashMap};

pub fn part_one() {
    let timer = Timer::new();
    let lines = include_bytes!("../inputs/day10.input")
        .split(|b| *b == b'\n').map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>();
    
    let mut closes : HashSet<u8> = HashSet::new();
    let mut opens  : HashSet<u8> = HashSet::new();
    
    let mut illegal : HashMap<u8, usize> = HashMap::new();
    illegal.insert(b')', 3);
    illegal.insert(b']', 57);
    illegal.insert(b'}', 1197);
    illegal.insert(b'>', 25137);
    
    let mut opener : HashMap<u8, u8> = HashMap::new();
    opener.insert(b')',b'(');
    opener.insert(b']',b'[');
    opener.insert(b'>',b'<');
    opener.insert(b'}',b'{');
    
    closes.insert(b')');
    closes.insert(b'}');
    closes.insert(b'>');
    closes.insert(b']');
    opens.insert(b'(');
    opens.insert(b'{');
    opens.insert(b'<');
    opens.insert(b'[');
    
    let mut score = 0;
    
    'outer: for line in lines {
        let mut stack : Vec<u8> = Vec::new();
        
        for c in line {
            if opens.contains(&c) {
                stack.push(c);
            } else {
                if stack.len() == 0 {
                    score += illegal.get(&c).unwrap();
                    continue 'outer;
                } else {
                    let p = stack.pop().unwrap();
                    if p < c && c-p <= 2 {
                        continue;
                    } else {
                        score += illegal.get(&c).unwrap();
                        continue 'outer;
                    }
                }
            }
        }
    }

    
    
    println!("day 10, part 1: {} in time {}us", score, timer.took().as_std().as_micros());    
}

pub fn part_two() {
    let timer = Timer::new();
    let lines = include_bytes!("../inputs/day10.input")
        .split(|b| *b == b'\n').map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>();

    let mut closes : HashSet<u8> = HashSet::new();
    let mut opens  : HashSet<u8> = HashSet::new();

    let mut illegal : HashMap<u8, usize> = HashMap::new();
    illegal.insert(b')', 3);
    illegal.insert(b']', 57);
    illegal.insert(b'}', 1197);
    illegal.insert(b'>', 25137);

    let mut good : HashMap<u8, usize> = HashMap::new();
    good.insert(b')', 1);
    good.insert(b']', 2);
    good.insert(b'}', 3);
    good.insert(b'>', 4);


    let mut opener : HashMap<u8, u8> = HashMap::new();
    opener.insert(b')',b'(');
    opener.insert(b']',b'[');
    opener.insert(b'>',b'<');
    opener.insert(b'}',b'{');

    let mut closer : HashMap<u8, u8> = HashMap::new();
    closer.insert(b'(',b')');
    closer.insert(b'[',b']');
    closer.insert(b'<',b'>');
    closer.insert(b'{',b'}');


    closes.insert(b')');
    closes.insert(b'}');
    closes.insert(b'>');
    closes.insert(b']');
    opens.insert(b'(');
    opens.insert(b'{');
    opens.insert(b'<');
    opens.insert(b'[');

    let mut scores : Vec<usize> = Vec::new();
    
    'outer: for line in lines {
        let mut stack : Vec<u8> = Vec::new();

        for c in line {
            if opens.contains(&c) {
                stack.push(c);
            } else {
                if stack.len() == 0 {
                    continue 'outer;
                } else {
                    let p = stack.pop().unwrap();
                    if p < c && c-p <= 2 {
                        continue;
                    } else {
                        continue 'outer;
                    }
                }
            }
        }
        
        let mut score : usize = 0;
        while stack.len() > 0 {
            let c = stack.pop().unwrap();
            score *= 5;
            score += good.get(closer.get(&c).unwrap()).unwrap();
        }
        scores.push(score);
    }

    scores.sort();
    
    let mid = scores.get(scores.len()/2).unwrap();

    println!("day 10, part 2: {} in time {}us", mid, timer.took().as_std().as_micros());
}