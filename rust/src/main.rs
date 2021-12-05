
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
    
    println!("day 1, part1: {}", inc_count);
    
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

    println!("day 1, part2: {}", inc_count);
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
    
    println!("day 2, part1: {}", h*d);
    
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

    println!("day 2, part1: {}", h*d);

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

fn main() {
    day_one_part_one();
    day_one_part_two();
    day_two_part_one();
    day_two_part_two();
    day_three_part_one();
    day_three_part_two();
}
