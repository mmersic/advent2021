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