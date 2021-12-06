pub fn part_one() {
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

fn bit_count_original(lines: &Vec<&str>) -> Vec<i32> {
    let mut v: Vec<i32> = vec!(0; 12);

    for line in &*lines {
        let b = line.as_bytes();
        (0..12).for_each(|i| {
            if b[i] == b'1' {
                v[i] += 1;
            }
        });
    }
    v
}


fn bit_count(lines: &Vec<&str>) -> (Vec<usize>, usize) {
    let mut v: Vec<usize> = vec!(0; 12);

    for line in &*lines {
        let b = line.as_bytes();
        (0..12).for_each(|i| {
            if b[i] == b'1' {
                v[i] += 1;
            }
        });
    }

    let mut rl_half = lines.len() / 2;
    if lines.len() % 2 == 1 {
        rl_half += 1;
    }

    (v, rl_half)
}

fn str_to_int(s : &str) -> u32 {
    return s.as_bytes().iter().fold(0, |mut acc : u32, v| {
        acc *= 2;
        if *v == b'1' { acc += 1; }
        acc
    });
}

fn str_to_int_original(s : &str) -> i32 {
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

pub fn part_two() {
    let lines = include_str!("../inputs/day3.input")
        .lines().collect::<Vec<&str>>();

    let one = b'1';
    let zero = b'0';

    let mut result : Vec<&str> = lines.to_vec();

    for i in 0 .. 12 {
        let v: Vec<i32> = bit_count_original(&result);
        let mut rl_half: usize = result.len()/2;
        if result.len() % 2 == 1 {
            rl_half += 1;
        }

        result = result.iter().filter(|x| (v[i] >= rl_half as i32 && x.as_bytes()[i] == one) || (v[i] < rl_half as i32 && x.as_bytes()[i] == zero)).map(|x| *x).collect::<Vec<&str>>();
        if result.len() == 1 {
            break;
        }
    }

    let o2 : i32 = str_to_int_original(&result[0]);

    let mut result : Vec<&str> = lines.to_vec();

    for i in 0 .. 12 {
        let v: Vec<i32> = bit_count_original(&result);
        let mut rl_half: usize = result.len()/2;
        if result.len() % 2 == 1 {
            rl_half += 1;
        }

        result = result.iter().filter(|x| (v[i] < rl_half as i32 && x.as_bytes()[i] == one) || (v[i] >= rl_half as i32 && x.as_bytes()[i] == zero)).map(|x| *x).collect::<Vec<&str>>();
        if result.len() == 1 {
            break;
        }
    }

    let co2 : i32 = str_to_int_original(&result[0]);
    
    println!("day 3, part 2: {}", o2*co2);
}

pub fn part_one_optimized() {
    let (g, e) = include_bytes!("../inputs/day3.input")
        .split(|b| *b == b'\n')
        .fold(vec!(0; 12), |mut v,s| {
            (0..12).for_each(|i| {
                if s[i] == b'1' {
                    v[i] += 1;
                }
            });
            v
        })
        .into_iter()
        .fold((0,0), |(mut g, mut e), s| {
            g *= 2; e *= 2;
            if s > 500 {
                g += 1;
            } else {
                e += 1;
            }
            (g, e)
        });
    
    println!("day 3, part 1: {}", g*e);
}

pub fn part_two_optimized() {
    let lines = include_str!("../inputs/day3.input")
        .lines().collect::<Vec<&str>>();
    
    let mut result = lines.to_vec();
    for i in 0 .. 12 {
        let (v, rl_half) = bit_count(&result);
        result = result.iter().filter(|x| (v[i] >= rl_half && x.as_bytes()[i] == b'1') || (v[i] < rl_half && x.as_bytes()[i] == b'0')).map(|x| *x).collect::<Vec<&str>>();
        if result.len() == 1 {
            break;
        }
    }
    
    let o2 = str_to_int(result[0]);

    result = lines.to_vec();
    for i in 0 .. 12 {
        let (v, rl_half) = bit_count(&result);
        result = result.iter().filter(|x| (v[i] < rl_half && x.as_bytes()[i] == b'1') || (v[i] >= rl_half && x.as_bytes()[i] == b'0')).map(|x| *x).collect::<Vec<&str>>();
        if result.len() == 1 {
            break;
        }
    }

    let co2 = str_to_int(result[0]);

    println!("day 3, part 2: {}", o2*co2);
}

//This is from:
//https://github.com/timvisee/advent-of-code-2021/blob/master/day03b/src/main.rs
pub fn part_two_final() {
    let nums = include_str!("../inputs/day3.input")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..12)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..12)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    println!("day 3, part 2: {}", oxy * co2);
}