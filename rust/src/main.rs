#![feature(array_windows)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn all() {
    day_01::part_one();
    day_01::part_one_optimized();
    day_01::part_one_final();
    day_01::part_two();
    day_01::part_two_optimized();
    day_01::part_two_final();
    day_02::part_one();
    day_02::part_one_optimized();
    day_02::part_one_final();
    day_02::part_two();
    day_02::part_two_optimized();
    day_03::part_one();
    day_03::part_two();
    day_04::part_one();
    day_04::part_two();
    day_05::part_one();
    day_05::part_two();
    day_05::part_one_optimized();
    day_05::part_two_optimized_one();
    day_05::part_two_optimized_two();
}

fn main() {
    all();
}

