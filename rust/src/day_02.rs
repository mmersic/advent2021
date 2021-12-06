pub fn part_one() {
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

pub fn part_one_optimized() {
    let mut h : usize = 0;
    let mut d : usize = 0;

    include_str!("../inputs/day2.input")
        .lines()
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .map(|x| (x[0], (x[1].parse::<usize>()).unwrap()))
        .for_each(|x| {
            match x.0 {
                "forward" => h += x.1,
                "down" => d += x.1,
                "up" => d -= x.1,
                _ => ()
            }
        });
    
    println!("day 2, part 1: {}", h*d);
}

//Mostly from:
//https://github.com/timvisee/advent-of-code-2021/blob/master/day02a/src/main.rs
pub fn part_one_final() {
    let (h, d) = include_str!("../inputs/day2.input")
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(x, y)| (x, y.parse::<usize>().unwrap()))
        .fold((0,0), |(h, d),  (orders, dist) | {
            match orders {
                "forward" => (h + dist, d),
                "down" => (h, d + dist),
                "up" => (h, d - dist),
                _ => unreachable!(),
            }
        });

    println!("day 2, part 1: {}", h*d);
}

pub fn part_two() {
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

pub fn part_two_optimized() {
    let (_, h, d) = include_str!("../inputs/day2.input")
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(x, y)| (x, y.parse::<usize>().unwrap()))
        .fold((0, 0, 0), |(a, h, d),  (orders, dist) | {
            match orders {
                "forward" => (a, h + dist, d + (dist * a)),
                "down" => (a + dist, h, d),
                "up" => (a - dist, h, d),
                _ => unreachable!(),
            }
        });

    println!("day 2, part 2: {}", h*d);
}