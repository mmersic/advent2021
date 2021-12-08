#![feature(array_windows)]
#![feature(drain_filter)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

#[allow(dead_code)]
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
    day_03::part_one_optimized();
    day_03::part_two();
    day_03::part_two_optimized();
    day_03::part_two_final();
    day_04::part_one();
    day_04::part_one_final();
    day_04::part_two();
    day_05::part_one();
    day_05::part_one_optimized();
    day_05::part_two();
    day_05::part_two_optimized_one();
    day_05::part_two_optimized_two();
    day_06::part_one();
    day_06::part_one_final();
    day_06::part_two();
    day_06::part_two_optimized();
    day_06::part_two_final();
    day_07::part_one();
    day_07::part_one_optimized();
    day_07::part_two();
    day_07::part_two_optimized();
}

fn main() {
    day_07::part_two();
    day_07::part_two_optimized();
}

