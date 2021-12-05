pub fn part_one() {
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

pub fn part_two() {
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