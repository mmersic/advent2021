#![feature(array_windows)]
#![feature(drain_filter)]
#![feature(int_abs_diff)]
#![feature(destructuring_assignment)]
#![feature(exact_size_is_empty)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

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
    day_08::part_one();
    day_08::part_one_optimized();
    day_08::part_one_final();
    day_08::part_two();
    day_08::part_two_optimized();
    day_09::part_one();
    day_09::part_one_optimized();
    day_09::part_two();
    day_09::part_two_optimized();
    day_10::part_one();
    day_10::part_two();
    day_11::part_one();
    day_11::part_two();
    day_12::part_one();
    day_12::part_two();
    day_13::part_one();
    day_13::part_two();
    day_13::part_two_optimized();
    day_14::part_one();
    day_14::part_two();
    day_15::part_one();
    day_15::part_two();
    day_16::part_one();
    day_16::part_two();
    day_16::part_two();
    day_16::part_two_streaming();
}

fn main() {
    day_17::part_one();
    day_17::part_two();
}

